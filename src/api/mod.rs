pub mod user;
pub mod xlive;

use anyhow::{bail, Result};
use log::{debug, trace};
use std::sync::Arc;

use crate::cache;
use serde::Serialize;

use self::user::User;
use self::xlive::Xlive;

struct ApiRequestBuilder {
    ctx: crate::Context,
    api_info_getter: Option<fn(&str) -> &serde_json::Value>,
}

struct ApiRequestParamsBuilder {
    ctx: crate::Context,
    builder: reqwest::RequestBuilder,
}

/// Store api request that is going to emit.
pub struct ApiRequest {
    ctx: crate::Context,
    builder: Option<reqwest::RequestBuilder>,
    bufferable: bool,
    invalidate_flag: bool,
}

impl ApiRequestBuilder {
    /// Create new ApiRequestBuilder, clone reqwest::Client
    pub fn new(ctx: &crate::Context) -> Self {
        Self {
            ctx: ctx.clone(),
            api_info_getter: None,
        }
    }

    /// Assign api info get function pointer
    pub fn api(mut self, f: fn(&str) -> &serde_json::Value) -> Self {
        self.api_info_getter = Some(f);
        self
    }

    /// Transform to ApiRequestParamsBuilder from api info path
    ///
    /// Require call method `api()` first. If not will panic.
    pub fn path<T: ToString>(self, p: T) -> ApiRequestParamsBuilder {
        let method_path = format!("{}/method", p.to_string());
        let url_path = format!("{}/url", p.to_string());
        self.path2(method_path, url_path)
    }

    pub fn path2<T: ToString, U: ToString>(
        self,
        method_path: T,
        url_path: U,
    ) -> ApiRequestParamsBuilder {
        let mpath = method_path.to_string();
        let upath = url_path.to_string();
        let method = self.get_from_path(&mpath);
        let url = self.get_from_path(&upath);
        self.into_params_builder(method, url)
    }

    fn get_from_path<'a>(&self, path: &'a str) -> &'a str {
        let getter = self
            .api_info_getter
            .expect("missed api info getter function");
        getter(path)
            .as_str()
            .unwrap_or_else(|| panic!("invalid api info at path {}", path))
    }

    /// Create ApiRequestParamsBuilder from method and url.
    fn into_params_builder(self, method: &str, url: &str) -> ApiRequestParamsBuilder {
        let builder = match method {
            "get" | "GET" => self.ctx.net.get(url),
            _ => panic!("unimpl method {}", method),
        };
        ApiRequestParamsBuilder {
            ctx: self.ctx,
            builder,
        }
    }
}

impl ApiRequestParamsBuilder {
    /// Pass parameters to `reqwest::RequestBuilder` .
    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> ApiRequestParamsBuilder {
        self.builder = self.builder.query(query);
        self
    }

    fn build(self, bufferable: bool) -> Result<ApiRequest> {
        Ok(ApiRequest {
            ctx: self.ctx.clone(),
            builder: Some(self.builder),
            bufferable,
            invalidate_flag: false,
        })
    }

    /// Finish build progress.
    pub fn bufferable(self) -> Result<ApiRequest> {
        self.build(true)
    }

    pub fn nobuffer(self) -> Result<ApiRequest> {
        self.build(false)
    }
}

impl ApiRequest {
    /// Do async api query, filter api response to extract result data.
    /// Use buffer if it is possible
    pub async fn query(self) -> Result<serde_json::Value> {
        let req = self.builder.expect("not null builder").build()?;
        let buffer_key = req.url().to_string();

        if self.bufferable && !self.invalidate_flag {
            if let Some(v) = self.ctx.cacher.cache_get(&buffer_key) {
                if let Ok(r) = serde_json::from_str(&v) {
                    debug!("read cache {}", buffer_key);
                    return Ok(r);
                }
            }
        }

        let resp_txt = self.ctx.net.execute(req).await?.text().await?;
        trace!("response text: {}", &resp_txt);
        let resp: serde_json::Value = match serde_json::from_str(&resp_txt) {
            Err(e) => {
                let s = &resp_txt[e.column() - 1..];
                trace!("try response slice: {}", s);
                serde_json::from_str(s)?
            }
            Ok(r) => r,
        };

        trace!("ok {}", buffer_key);
        let r = Self::filter_result(resp)?;

        if self.bufferable {
            debug!("update cache {}", buffer_key);
            self.ctx.cacher.cache_store(&buffer_key, &r.to_string());
        }

        Ok(r)
    }

    fn filter_result(mut resp: serde_json::Value) -> Result<serde_json::Value> {
        if let Some(code) = resp["code"].as_i64() {
            if code != 0 {
                let msg = resp["msg"]
                    .as_str()
                    .or_else(|| resp["message"].as_str())
                    .unwrap_or("detail missed");
                bail!("bad resp, code:{} msg:{}", code, msg)
            } else {
                Ok(if resp["data"].is_null() {
                    resp["result"].take()
                } else {
                    resp["data"].take()
                })
            }
        } else {
            bail!("empty response")
        }
    }

    /// Invalidate cache
    pub fn invalidate(mut self) -> Self {
        self.invalidate_flag = true;
        self
    }
}

impl Clone for ApiRequest {
    fn clone(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            builder: self.builder.as_ref().map(|b| {
                b.try_clone()
                    .expect("Api request must not have stream body")
            }),
            bufferable: self.bufferable,
            invalidate_flag: self.invalidate_flag,
        }
    }
}

/// API client context
#[derive(Clone)]
pub struct Context {
    net: reqwest::Client,
    cacher: Arc<dyn cache::Cacher + Send + Sync>,
}

impl Context {
    /// New a context
    ///
    /// Default with cacher [`SimpleMemCacher`][crate::cache::SimpleMemCacher].
    pub fn new() -> Result<Self> {
        Ok(Self {
            net: new_http_client()?,
            cacher: Arc::new(cache::NoCacher::default()),
        })
    }

    /// Enable to replace the cacher
    pub fn replace_cacher(self, cacher: Arc<dyn cache::Cacher + Send + Sync>) -> Self {
        Self { cacher, ..self }
    }

    /// New a user API collection
    pub fn new_user<T: ToString>(&self, uid: T) -> User {
        User::new(self, uid)
    }

    /// New a live neXt generation API collection
    pub fn xlive(&self) -> Xlive {
        Xlive::new(self)
    }

    fn req_build(&self) -> ApiRequestBuilder {
        ApiRequestBuilder::new(self)
    }
}

fn new_http_client() -> Result<reqwest::Client> {
    Ok(reqwest::ClientBuilder::new()
        .user_agent("Mozilla/5.0")
        .referer(false)
        .default_headers({
            let mut hdrs = reqwest::header::HeaderMap::new();
            hdrs.insert(
                "Referer",
                reqwest::header::HeaderValue::from_static("https://www.bilibili.com"),
            );
            hdrs
        })
        .build()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_http_client() -> Result<()> {
        new_http_client()?;
        Ok(())
    }

    #[test]
    #[should_panic(expected = "missed api info getter function")]
    fn test_panic_api_req_builder_not_set_api() {
        let n = crate::Context::new().unwrap();
        let _ = ApiRequestBuilder::new(&n).path("info/info");
    }

    #[test]
    #[should_panic(expected = "invalid api info at path bad/bad/bad")]
    fn test_panic_api_req_builder_wrong_path() {
        let n = crate::Context::new().unwrap();
        let _ = ApiRequestBuilder::new(&n)
            .api(crate::api_info::user::get)
            .path("bad/bad/bad");
    }
}
