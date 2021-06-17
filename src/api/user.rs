use crate::net::{self, NetApi};
use crate::NetContext;
use crate::api_info;

pub struct User {
    ctx: net::MethodDispatcher,
    uid: String,
}

impl User {
    pub fn new<T>(n: &NetContext, uid: T) -> Self
        where String: From<T>
    {
        Self {
            uid: String::from(uid),
            ctx: n.ctx.clone(),
        }
    }

    pub async fn get_info(&self) -> net::RetValue {
        Ok(self.ctx.api(api_info::user::get("info/info"))
            .query(&[("uid", &self.uid)])
            .api_call()
            .result()
            .await?)
    }
}
