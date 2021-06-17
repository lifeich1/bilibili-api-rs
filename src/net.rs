use crate::error::Error;

#[derive(Clone)]
pub struct MethodDispatcher(reqwest::Client);

impl MethodDispatcher {
    fn method(&self, method: &str, url: &str) -> reqwest::RequestBuilder {
        match method {
            "GET" => self.0.get(url),
            _ => panic!("net: unimplemented method: {}", method),
        }
    }

    pub fn api(&self, info: &serde_json::Value) -> reqwest::RequestBuilder {
        let method = info["method"]
            .as_str()
            .expect(&format!("net: api info invalid method: {:?}", info));
        let url = info["url"]
            .as_str()
            .expect(&format!("net: api info invalid method: {:?}", info));
        self.method(method, url)
    }
}

pub fn new_net_context() -> crate::Result<MethodDispatcher> {
    Ok(MethodDispatcher(
        reqwest::ClientBuilder::new()
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
            .build()?,
    ))
}

pub struct NetApiCall {
    req: reqwest::RequestBuilder,
}

pub trait NetApi {
    fn api_call(self) -> NetApiCall;
}

impl NetApi for reqwest::RequestBuilder {
    fn api_call(self) -> NetApiCall {
        NetApiCall { req: self }
    }
}

pub type RetValue = crate::Result<serde_json::Value>;

impl NetApiCall {
    pub async fn result(self) -> RetValue {
        let mut resp = self.req.send().await?.json::<serde_json::Value>().await?;
        if let Some(code) = resp["code"].as_i64() {
            if code != 0 {
                let msg = resp["msg"]
                    .as_str()
                    .or(resp["message"].as_str())
                    .unwrap_or("detail missed");
                Err(Error::remote_err(format!(
                    "api return code {}: {}",
                    code, msg
                )))
            } else {
                Ok(if resp["data"].is_null() {
                    resp["result"].take()
                } else {
                    resp["data"].take()
                })
            }
        } else {
            Err(Error::remote_err("api return code null"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn real_test_1n_get_video_info() -> crate::Result<()> {
        let j = new_net_context()?
            .method("GET", "https://api.bilibili.com/x/web-interface/view")
            .query(&[("bvid", "BV1uv411q7Mv")])
            .api_call()
            .result()
            .await?;
        println!("data: {}\n", j.to_string());
        assert!(!j.is_null());

        Ok(())
    }

    #[test]
    fn test_new_net_context() -> crate::Result<()> {
        new_net_context()?;
        Ok(())
    }
}
