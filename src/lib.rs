pub mod api;
mod api_info;
pub mod cache;
pub mod error;
mod net;

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
            net: net::new_http_client()?,
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
