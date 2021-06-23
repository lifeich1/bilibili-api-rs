use crate::api_info;
use crate::net::ApiRequestBuilder;
use crate::Context;
use super::ApiCall;

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
        self.ctx.req_build()
            .api(api_info::user::get)
    }

    pub fn get_info(&self) -> ApiCall {
        self.rb()
            .path("info/info")
            .query(&[("mid", &self.uid)])
            .bufferable()
    }

    pub fn video_list(&self, page_no: i32) -> ApiCall {
        let pn = page_no.to_string();
        self.rb()
            .path("info/video")
            .query(&[
                   ("mid", &self.uid),
                   ("pn", &pn),
            ])
            .query(&[
                   ("tid", "0"),
                   ("ps", "30"),
                   ("order", "pubdate"),
            ])
            .nobuffer()
    }
}
