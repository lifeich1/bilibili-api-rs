pub mod api;
mod api_info;
pub mod error;
mod net;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

#[derive(Clone)]
pub struct Context {
    net: net::MethodDispatcher,
}

impl Context {
    pub fn new() -> crate::Result<Self> {
        Ok(Self {
            net: net::new_net_context()?,
        })
    }

    pub fn new_user<T: ToString>(&self, uid: T) -> api::User {
        api::User::new(self, uid)
    }
}
