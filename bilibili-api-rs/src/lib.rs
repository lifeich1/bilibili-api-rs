#![warn(missing_docs)]
//! bilibili-api-rs is a rust library project got inspiration from [bilibili-api](https://github.com/Passkou/bilibili-api).
//!
//! Currently "GET" apis only. Api interface `User`, `Xlive` derive from
//! [Client][crate::wbi::Client].
//!
//! Api result is part of response, alike [bilibili-api](https://github.com/Passkou/bilibili-api),
//! is response["data"]. Invalid response treated as error then bail.
//!
//! *High overhead*: to anti-detect, client open one whole new connection in every request.
//!
//! ## Example
//! ```
//! use bilibili_api_rs::Client;
//! use anyhow::Result;
//! async fn test_xlive() -> Result<()> {
//!     let cli = Client::new();
//!     let xlive = cli.xlive(/*virtual*/9, /*all*/0);
//!     let lives = xlive.list(2).await?;
//!     Ok(())
//! }
//! ```
use log::debug;
use std::sync::Arc;
use tokio::sync::mpsc;

pub mod wbi;
pub use wbi::Client;

type StateData = im::HashMap<String, String>;
type Json = serde_json::Value;

#[derive(Clone, Debug)]
struct Bench {
    data: Arc<Json>,
    state: StateData,
    tx: mpsc::Sender<StateData>,
}

impl Bench {
    pub fn new() -> (Self, mpsc::Receiver<StateData>) {
        let user: Json = serde_json::from_str(include_str!("api_info/user.json"))
            .expect("api_info/user.json invalid");
        let live: Json = serde_json::from_str(include_str!("api_info/live.json"))
            .expect("api_info/live.json invalid");
        let video: Json = serde_json::from_str(include_str!("api_info/video.json"))
            .expect("api_info/video.json invalid");
        let api_xlive: Json = serde_json::from_str(include_str!("api_info/xlive.json"))
            .expect("api_info/xlive.json invalid");
        let credential: Json = serde_json::from_str(include_str!("api_info/credential.json"))
            .expect("api_info/credential.json invalid");
        let wbi_oe: Json =
            serde_json::from_str(include_str!("wbi_oe.json")).expect("wbi_oe.json invalid"); // this file has to be
                                                                                             // manually maintain now,
                                                                                             // perl tool TODO
        let buvid3: String = uuid::Uuid::now_v1(&[99, 11, 16, 32, 1, 7]).to_string();
        let state = StateData::unit("buvid3".into(), buvid3);
        let (tx, rx) = mpsc::channel(1);
        (
            Self {
                data: Arc::new(serde_json::json!({
                    "api": {
                        "user": user,
                        "live": live,
                        "video": video,
                        "xlive": api_xlive,
                        "credential": credential,
                    },
                    "cookie_state": [
                        "buvid3"
                    ],
                    "wbi_oe": wbi_oe,
                    "headers": {
                        "REFERER":  "https://www.bilibili.com",
                        "USER_AGENT": "Mozilla/5.0",
                    }
                })),
                state,
                tx,
            },
            rx,
        )
    }

    pub fn commit_state(&self, change: impl Fn(&mut StateData)) {
        let mut s = self.state.clone();
        change(&mut s);
        if let Err(e) = self.tx.try_send(s) {
            debug!("bench try_send {e:?}");
        }
    }
}

/// Lodash-like get helper, implemented for `serde_json`
///
/// ```
/// use bilibili_api_rs::Lodash;
/// # use serde_json::json;
/// let v = json!({
///     "following": [ {
///         "mid": 1472906636,
///         "name": "ywwuyi",
///     }, {
///         "mid": 15810,
///         "name": "Mr.Quin",
///     }],
/// });
/// assert_eq!(v.at(json!(["following", 0, "mid"])), json!(1472906636));
/// assert_eq!(v["following"].at(json!([
///         [0, "name"],
///         [1, "name"],
///     ])),
///     json!(["ywwuyi", "Mr.Quin"]));
/// ```
pub trait Lodash {
    /// Input a matrix, output a vector; input a vector, output single one value
    #[must_use]
    fn at(&self, paths: Json) -> Self;
}

fn lodash_step<'a>(v: &'a Json, p: &Json) -> &'a Json {
    match p {
        Json::Number(n) => &v[n.as_u64().map(usize::try_from).unwrap().unwrap()],
        Json::String(s) => &v[s],
        _ => &Json::Null,
    }
}

fn lodash_get<'a>(v: &'a Json, path: &Json) -> &'a Json {
    let Json::Array(path) = path else {
        return &Json::Null;
    };
    if path.is_empty() {
        return v;
    }
    let mut it = path.iter();
    let mut v = lodash_step(v, it.next().unwrap());
    for p in it {
        v = lodash_step(v, p);
    }
    v
}

impl Lodash for Json {
    fn at(&self, paths: Json) -> Self {
        let Some(a) = paths.as_array() else {
            return Self::Null;
        };
        if a[0].is_array() {
            let mut v: Vec<Self> = Vec::new();
            for path in a {
                v.push(lodash_get(self, path).clone());
            }
            Self::Array(v)
        } else {
            lodash_get(self, &paths).clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::thread;

    #[test]
    fn test_lodash_at() {
        let bench = Bench::new().0;
        let v = &bench.data;
        assert_eq!(
            v.at(json!([
                ["headers", "REFERER"],
                ["headers", "__must_null__"],
                ["cookies", "SESSDATA"],
            ])),
            json!(["https://www.bilibili.com", (), ""])
        );
        assert_eq!(
            v.at(json!(["headers", "REFERER"])),
            json!("https://www.bilibili.com")
        );
    }

    #[test]
    fn validate_wbi_user_info() {
        let bench = Bench::new().0;
        assert_eq!(
            bench.data["api"]["user"]["info"]["info"]["wbi"].as_bool(),
            Some(true)
        );
    }

    #[test]
    fn validate_method_xlive_get_list() {
        let bench = Bench::new().0;
        assert_eq!(
            bench.data["api"]["xlive"]["info"]["get_list"]["method"].as_str(),
            Some("GET")
        );
    }

    fn json_state(bench: &Bench) -> Json {
        serde_json::to_value(bench.state.clone()).unwrap()
    }

    #[test]
    fn commit_state() {
        let bench = Bench::new().0;
        assert_eq!(json_state(&bench), json!({}));
        bench.commit_state(|s| {
            s.insert("test".into(), "value".into());
        });
        assert_eq!(json_state(&bench), json!({"test":"value"}));
        bench.commit_state(|s| {
            s.insert("test".into(), "modified".into());
        });
        assert_eq!(json_state(&bench), json!({"test":"modified"}));
    }

    #[test]
    fn multithread_commit_state() {
        let bench0 = Bench::new().0;
        assert_eq!(json_state(&bench0), json!({}));

        let bench = bench0.clone();
        let hdl = thread::spawn(move || {
            bench.commit_state(|s| {
                s.insert("test".into(), "value".into());
            });
        });
        assert!(hdl.join().is_ok());
        assert_eq!(json_state(&bench0), json!({"test":"value"}));

        let bench = bench0.clone();
        let hdl = thread::spawn(move || {
            bench.commit_state(|s| {
                s.insert("test".into(), "modified".into());
            });
        });
        assert!(hdl.join().is_ok());
        assert_eq!(json_state(&bench0), json!({"test":"modified"}));
    }

    #[test]
    fn insure_get_nav_api_no_encwbi() {
        let bench = Bench::new().0;
        assert!(matches!(
            bench.data["api"]["credential"]["valid"]["wbi"],
            Json::Null | Json::Bool(false)
        ));
    }
}
