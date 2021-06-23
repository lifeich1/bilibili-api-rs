use crate::api_info;
use crate::net::ApiRequest;
use crate::error::ApiResult;
use crate::Context;

pub struct User {
    ctx: Context,
    uid: String,
}

impl User {
    pub fn new<T: ToString>(n: &Context, uid: T) -> Self {
        Self {
            uid: uid.to_string(),
            ctx: n.clone(),
        }
    }

    pub fn get_info(&self) -> ApiResult<ApiRequest> {
        self.ctx.req_build()
            .api(api_info::user::get)
            .path("info/info")
            .query(&[("mid", &self.uid)])
            .bufferable()
    }
}
