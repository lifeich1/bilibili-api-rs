//! bilibili-api-rs is a rust library project got inspiration from [bilibili-api](https://github.com/Passkou/bilibili-api).
//!
//! ## Example
use rpds::{RedBlackTreeMap, RedBlackTreeMapSync};
use std::sync::{Arc, RwLock};

pub mod wbi;

type StateData = RedBlackTreeMapSync<String, String>;
type Json = serde_json::Value;

#[derive(Clone, Debug)]
struct Bench {
    data_: Json,
    state_: Arc<RwLock<StateData>>,
    base_state_: StateData,
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
                "headers": {
                    "REFERER":  "https://www.bilibili.com",
                    "USER_AGENT": "Mozilla/5.0",
                }
            }),
            state_: Arc::new(RwLock::new(RedBlackTreeMap::new_sync())),
            base_state_: StateData::default(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::thread;

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

        let mut bench = bench0.clone();
        let hdl = thread::spawn(move || {
            bench.commit_state(|s| s.insert("test".into(), "value".into()));
        });
        assert!(hdl.join().is_ok());
        assert_eq!(json_state(&mut bench0), json!({"test":"value"}));

        let mut bench = bench0.clone();
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
