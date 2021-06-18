use crate::api_info;
use crate::net::{self, NetApi};
use crate::NetContext;

pub struct User {
    ctx: net::MethodDispatcher,
    uid: String,
}

impl User {
    pub fn new<T: ToString>(n: &NetContext, uid: T) -> Self {
        Self {
            uid: uid.to_string(),
            ctx: n.ctx.clone(),
        }
    }

    pub async fn get_info(&self) -> net::RetValue {
        self.ctx
            .api(api_info::user::get("info/info"))
            .query(&[("mid", &self.uid)])
            .api_call()
            .result()
            .await
    }
}
