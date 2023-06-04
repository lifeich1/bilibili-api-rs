//! bilibili-api-rs is a rust library project got inspiration from [bilibili-api](https://github.com/Passkou/bilibili-api).
//!
//! ## Example

struct Bench {
    pub data: serde_json::Value,
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
            data: serde_json::json!({
                "api": {
                    "user": user,
                    "live": live,
                    "video": video,
                    "xlive": xlive,
                }
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_wbi_user_info() {
        let bench = Bench::new();
        assert_eq!(
            bench.data["api"]["user"]["info"]["info"]["wbi"].as_bool(),
            Some(true)
        );
    }

    #[test]
    fn validate_method_xlive_get_list() {
        let bench = Bench::new();
        assert_eq!(
            bench.data["api"]["xlive"]["info"]["get_list"]["method"].as_str(),
            Some("GET")
        );
    }
}
