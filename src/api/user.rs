use crate::api_info;
use crate::net::{self, NetApi};
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

    pub async fn get_info(&self) -> net::RetValue {
        self.ctx
            .net
            .api(api_info::user::get("info/info"))
            .query(&[("mid", &self.uid)])
            .api_call()
            .result()
            .await
    }
}
