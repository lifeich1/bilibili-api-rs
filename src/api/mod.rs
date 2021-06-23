pub mod user;

pub use self::user::User;

pub type ApiCall = crate::error::ApiResult<crate::net::ApiRequest>;
