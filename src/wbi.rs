// TODO encwbi
// TODO generic do request
use super::Bench;
use anyhow::Result;
use reqwest::header::{REFERER, USER_AGENT};

type Json = serde_json::Value;

async fn do_req(bench: &Bench, api_path: Vec<&str>, opts: Json) -> Result<Json> {
    let data = bench.data();
    let cli = reqwest::Client::new();
    let mut api = &data["api"];
    for p in api_path {
        api = &api[p];
    }
    let req = cli
        .request(
            api["method"].as_str().unwrap().parse().unwrap(),
            api["url"].as_str().unwrap(),
        )
        .header(
            REFERER,
            data["headers"]["REFERER"]
                .as_str()
                .unwrap()
                .parse::<reqwest::header::HeaderValue>()
                .unwrap(),
        )
        .header(
            USER_AGENT,
            data["headers"]["USER_AGENT"]
                .as_str()
                .unwrap()
                .parse::<reqwest::header::HeaderValue>()
                .unwrap(),
        )
        .query(&opts["query"]);
    Ok(serde_json::from_str(&req.send().await?.text().await?)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_do_req() -> Result<()> {
        let res = do_req(
            &Bench::new(),
            vec!["xlive", "info", "get_list"],
            json!({
                "query": {
                    "platform": "web",
                    "parent_area_id": 9,
                    "area_id": 0,
                    "sort_type": "sort_type_291",
                    "page": 2
                }
            }),
        )
        .await?;
        println!("res: {:?}", &res);
        assert_eq!(res["code"].as_i64(), Some(0));
        Ok(())
    }
}
