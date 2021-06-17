mod api_info;
pub mod error;
mod net;
pub mod api;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub struct NetContext {
    ctx: net::MethodDispatcher,
}

impl NetContext {
    pub fn new() -> crate::Result<Self> {
        Self { ctx: net::new_net_context()? }
    }
}
