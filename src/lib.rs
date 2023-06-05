//! bilibili-api-rs is a rust library project got inspiration from [bilibili-api](https://github.com/Passkou/bilibili-api).
//!
//! ## Example
use rpds::{RedBlackTreeMap, RedBlackTreeMapSync};
use std::sync::{Arc, RwLock};

pub mod wbi;

type StateData = RedBlackTreeMapSync<String, String>;

#[derive(Clone, Debug)]
struct Bench {
    data_: serde_json::Value,
    state_: Arc<RwLock<StateData>>,
    base_state_: StateData,
}

impl Bench {
    pub fn new() -> Self {
        let user: serde_json::Value = serde_json::from_str(include_str!("api_info/user.json"))
            .expect("api_info/user.json invalid");
        let live: serde_json::Value = serde_json::from_str(include_str!("api_info/live.json"))
            .expect("api_info/live.json invalid");
        let video: serde_json::Value = serde_json::from_str(include_str!("api_info/video.json"))
            .expect("api_info/video.json invalid");
        let xlive: serde_json::Value = serde_json::from_str(include_str!("api_info/xlive.json"))
            .expect("api_info/xlive.json invalid");
        Self {
            data_: serde_json::json!({
                "api": {
                    "user": user,
                    "live": live,
                    "video": video,
                    "xlive": xlive,
                }
            }),
            state_: Arc::new(RwLock::new(RedBlackTreeMap::new_sync())),
            base_state_: StateData::default(),
        }
    }

    pub fn data(&self) -> &serde_json::Value {
        &self.data_
    }

    fn fetch_state(&mut self) {
        self.base_state_ = self
            .state_
            .read()
            .expect("persistent state should be always valid")
            .clone();
    }

    pub fn state(&mut self) -> serde_json::Value {
        self.fetch_state();
        serde_json::to_value(self.base_state_.clone())
            .expect("persistent state should be always json-like struct")
    }

    pub fn commit_state(&mut self, change: impl Fn(StateData) -> StateData) {
        loop {
            self.fetch_state();
            let fastforward = change(self.base_state_.clone());
            let mut guard = self
                .state_
                .write()
                .expect("easy sync in commit should not be poisoned");
            if self.base_state_ == *guard {
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

    #[test]
    fn commit_state() {
        let mut bench = Bench::new();
        assert_eq!(bench.state(), json!({}));
        bench.commit_state(|s| s.insert("test".to_string(), "value".to_string()));
        assert_eq!(bench.state(), json!({"test":"value"}));
        bench.commit_state(|s| s.insert("test".to_string(), "modified".to_string()));
        assert_eq!(bench.state(), json!({"test":"modified"}));
    }

    #[test]
    fn multithread_commit_state() {
        let mut bench0 = Bench::new();
        assert_eq!(bench0.state(), json!({}));

        let mut bench = bench0.clone();
        let hdl = thread::spawn(move || {
            bench.commit_state(|s| s.insert("test".to_string(), "value".to_string()));
        });
        assert!(hdl.join().is_ok());
        assert_eq!(bench0.state(), json!({"test":"value"}));

        let mut bench = bench0.clone();
        let hdl = thread::spawn(move || {
            bench.commit_state(|s| s.insert("test".to_string(), "modified".to_string()));
        });
        assert!(hdl.join().is_ok());
        assert_eq!(bench0.state(), json!({"test":"modified"}));
    }
}
