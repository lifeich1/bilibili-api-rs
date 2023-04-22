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

/// Declare error enum
pub mod error;

/// The module contain a helpful bevy plugin [`plugin::ApiRuntimePlugin`].
///
/// ## Usage
/// 1. Setup: create [`Runtime`][tokio::runtime::Runtime] and [`Context`][crate::Context] for
///    plugin, then add [ApiRuntimePlugin][plugin::ApiRuntimePlugin] to app with the runtime.
/// 2. Emit task: create a `ApiResult<ApiRequest>`, emit it with
///    [`ApiRequestTag`][plugin::ApiRequestTag] to [`ApiRequestEvent`][plugin::ApiRequestEvent]
///    channel.
/// 3. Receive result: [`ApiTaskResultEvent`][plugin::ApiTaskResultEvent] channel will offer the
///    tagged result.
///
/// ## Example
/// Checkout [`examples/bevy-task.rs`](https://github.com/lifeich1/bilibili-api-rs/blob/master/examples/bevy-task.rs)
#[cfg(feature = "plugin")]
pub mod plugin;

pub use api::{user::User, xlive, ApiRequest, Context};
pub use error::ApiResult;
