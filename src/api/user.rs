use crate::api::ApiRequest;
use crate::api_info;
use crate::error::ApiResult;
use crate::Context;

use super::ApiRequestBuilder;

/// User related APIs collection
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

    fn rb(&self) -> ApiRequestBuilder {
        self.ctx.req_build().api(api_info::user::get)
    }

    /// Get user basic info.
    pub fn get_info(&self) -> ApiResult<ApiRequest> {
        self.rb()
            .path("info/info")
            .query(&[("mid", &self.uid)])
            .bufferable()
    }

    /// Fetch user's upload video list.
    pub fn video_list(&self, page_no: i32) -> ApiResult<ApiRequest> {
        let pn = page_no.to_string();
        self.rb()
            .path("info/video")
            .query(&[("mid", &self.uid), ("pn", &pn)])
            .query(&[("tid", "0"), ("ps", "30"), ("order", "pubdate")])
            .nobuffer()
    }
}
