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

/// Provide bilibili APIs
pub mod api;

mod api_info;

/// The module declare the cache interface and provide a [`SimpleMemCacher`][cache::SimpleMemCacher]
pub mod cache;

pub use api::{user::User, xlive, Context};
