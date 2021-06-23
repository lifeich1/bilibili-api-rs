pub mod api;
mod api_info;
pub mod cache;
pub mod error;
pub mod net;

use std::sync::Arc;

pub use crate::error::ApiResult;

#[derive(Clone)]
pub struct Context {
    net: reqwest::Client,
    cacher: Arc<dyn cache::Cacher>,
}

impl Context {
    pub fn new() -> crate::ApiResult<Self> {
        Ok(Self {
            net: new_http_client()?,
            cacher: Arc::new(cache::SimpleMemCacher::default()),
        })
    }

    pub fn replace_cacher(self, cacher: Arc<dyn cache::Cacher>) -> Self {
        Self {
            cacher,
            ..self
        }
    }

    pub fn new_user<T: ToString>(&self, uid: T) -> api::User {
        api::User::new(self, uid)
    }

    fn req_build(&self) -> net::ApiRequestBuilder {
        net::ApiRequestBuilder::new(self)
    }
}


fn new_http_client() -> crate::ApiResult<reqwest::Client> {
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
    fn test_new_http_client() -> crate::Result<()> {
        new_http_client()?;
        Ok(())
    }
}
