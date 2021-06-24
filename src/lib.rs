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
///
/// ## Usage
/// 1. Setup: create a [`tokio::runtime::Runtime`] and add [`plugin::ApiRuntimePlugin`] to app with
///    the runtime.
/// 2. Emit task: create a `ApiResult<ApiRequest>`, spawn it with
///    [`spawn_on`][`plugin::SpawnOnWorld`] then attach the returned [`plugin::ApiRequestTask`] to
///    an entity.
/// 3. Receive result: the entity attached with `ApiRequestTask` will receive a
///    [`plugin::ApiTaskResult`].
#[cfg(feature = "plugin")]
pub mod plugin;

pub use api::{user::User, ApiRequest, Context};
pub use error::ApiResult;
