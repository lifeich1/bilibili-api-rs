//! bilibili-api-rs is a rust library project got inspiration from [bilibili-api](https://github.com/Passkou/bilibili-api).
//!
//! ## Example
//! ```
//! use bilibili_api_rs::Context;
//!
//! #[tokio::main]
//! async fn main() {
//!     let n = Context::new().unwrap();
//!     let v = n.new_user(15810).get_info().unwrap().query().await.unwrap();
//!     assert_eq!(v["name"].as_str().unwrap(), "Mr.Quin");
//! }
//! ```


pub mod api;
mod api_info;
pub mod cache;
pub mod error;

/// The module contain a helpful bevy plugin [`plugin::ApiRuntimePlugin`].
#[cfg(feature = "plugin")]
pub mod plugin;

pub use error::ApiResult;
pub use api::{Context, ApiRequest, user::User};
