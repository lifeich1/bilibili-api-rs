//! bilibili-api-rs is a rust library project got inspiration from [bilibili-api](https://github.com/Passkou/bilibili-api).
//!
//! Currently "GET" apis only. Api interface `User`, `Xlive` derive from
//! [Client][crate::wbi::Client].
//!
//! Api result is part of response, alike [bilibili-api](https://github.com/Passkou/bilibili-api),
//! is response["data"]. Invalid response treated as error then bail.
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
use rpds::{RedBlackTreeMap, RedBlackTreeMapSync};
use std::sync::{Arc, RwLock};

pub mod wbi;
pub use wbi::Client;

type StateData = RedBlackTreeMapSync<String, String>;
type Json = serde_json::Value;

#[derive(Clone, Debug)]
struct Bench {
    data_: Json,
    state_: Arc<RwLock<StateData>>,
}

impl Bench {
    pub fn new() -> Self {
        let user: Json = serde_json::from_str(include_str!("api_info/user.json"))
            .expect("api_info/user.json invalid");
        let live: Json = serde_json::from_str(include_str!("api_info/live.json"))
            .expect("api_info/live.json invalid");
        let video: Json = serde_json::from_str(include_str!("api_info/video.json"))
            .expect("api_info/video.json invalid");
        let xlive: Json = serde_json::from_str(include_str!("api_info/xlive.json"))
            .expect("api_info/xlive.json invalid");
        let credential: Json = serde_json::from_str(include_str!("api_info/credential.json"))
            .expect("api_info/credential.json invalid");
        let wbi_oe: Json =
            serde_json::from_str(include_str!("wbi_oe.json")).expect("wbi_oe.json invalid"); // this file has to be
                                                                                             // manually maintain now,
                                                                                             // perl tool TODO
        let buvid3: String = uuid::Uuid::now_v1(&[99, 11, 16, 32, 1, 7]).to_string();
        Self {
            data_: serde_json::json!({
                "api": {
                    "user": user,
                    "live": live,
                    "video": video,
                    "xlive": xlive,
                    "credential": credential,
                },
                "wbi_oe": wbi_oe,
                "cookies" : {
                    "buvid3": buvid3,
                    "SESSDATA": "", "bili_jct": "", "DedeUserID": "",
                },
                "headers": {
                    "REFERER":  "https://www.bilibili.com",
                    "USER_AGENT": "Mozilla/5.0",
                }
            }),
            state_: Arc::new(RwLock::new(RedBlackTreeMap::new_sync())),
        }
    }

    pub fn data(&self) -> &Json {
        &self.data_
    }

    pub fn state(&self) -> StateData {
        self.state_
            .read()
            .expect("persistent state should be always valid")
            .clone()
    }

    pub fn commit_state(&self, change: impl Fn(StateData) -> StateData) {
        loop {
            let base = self.state();
            let fastforward = change(base.clone());
            let mut guard = self
                .state_
                .write()
                .expect("easy sync in commit should not be poisoned");
            if base == *guard {
                *guard = fastforward;
                break;
            }
        }
    }
}

/// Lodash-like get helper, implemented for serde_json
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
    fn at(&self, paths: Json) -> Self;
}

fn lodash_step<'a>(v: &'a Json, p: &Json) -> &'a Json {
    match p {
        Json::Number(n) => &v[n.as_i64().unwrap() as usize],
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
            return Json::Null;
        };
        if a[0].is_array() {
            let mut v: Vec<Json> = Vec::new();
            for path in a {
                v.push(lodash_get(self, path).clone())
            }
            Json::Array(v)
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
        let bench = Bench::new();
        let v = bench.data();
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
        let bench = Bench::new();
        assert_eq!(
            bench.data()["api"]["user"]["info"]["info"]["wbi"].as_bool(),
            Some(true)
        );
    }

    #[test]
    fn validate_method_xlive_get_list() {
        let bench = Bench::new();
        assert_eq!(
            bench.data()["api"]["xlive"]["info"]["get_list"]["method"].as_str(),
            Some("GET")
        );
    }

    fn json_state(bench: &mut Bench) -> Json {
        serde_json::to_value(bench.state()).unwrap()
    }

    #[test]
    fn commit_state() {
        let mut bench = Bench::new();
        assert_eq!(json_state(&mut bench), json!({}));
        bench.commit_state(|s| s.insert("test".into(), "value".into()));
        assert_eq!(json_state(&mut bench), json!({"test":"value"}));
        bench.commit_state(|s| s.insert("test".into(), "modified".into()));
        assert_eq!(json_state(&mut bench), json!({"test":"modified"}));
    }

    #[test]
    fn multithread_commit_state() {
        let mut bench0 = Bench::new();
        assert_eq!(json_state(&mut bench0), json!({}));

        let bench = bench0.clone();
        let hdl = thread::spawn(move || {
            bench.commit_state(|s| s.insert("test".into(), "value".into()));
        });
        assert!(hdl.join().is_ok());
        assert_eq!(json_state(&mut bench0), json!({"test":"value"}));

        let bench = bench0.clone();
        let hdl = thread::spawn(move || {
            bench.commit_state(|s| s.insert("test".into(), "modified".into()));
        });
        assert!(hdl.join().is_ok());
        assert_eq!(json_state(&mut bench0), json!({"test":"modified"}));
    }

    #[test]
    fn insure_get_nav_api_no_encwbi() {
        let bench = Bench::new();
        assert!(matches!(
            bench.data()["api"]["credential"]["valid"]["wbi"],
            Json::Null | Json::Bool(false)
        ));
    }
}
