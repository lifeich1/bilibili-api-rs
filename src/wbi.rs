// TODO encwbi
// TODO generic do request
use super::Bench;
use anyhow::{bail, Result};
use reqwest::header::{REFERER, USER_AGENT};
use serde_json::json;

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
            api["method"].as_str().unwrap_or("GET").parse().unwrap(),
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

async fn fetch_wbi_salt(bench: &Bench) -> Result<()> {
    let nav = do_req(bench, vec!["credential", "valid"], json!({})).await?;
    let Some(imgurl) = nav["data"]["wbi_img"]["img_url"].as_str() else {
        bail!("fetch_wbi_salt: wbi_img/img_url invalid");
    };
    let Some(suburl) = nav["data"]["wbi_img"]["sub_url"].as_str() else {
        bail!("fetch_wbi_salt: wbi_img/sub_url invalid");
    };
    let ae: String = imgurl.to_owned() + suburl;
    let oe: Vec<i64> = bench.data()["wbi_oe"]
        .as_array()
        .expect("wbi_oe not array")
        .iter()
        .map(|v| v.as_i64().expect("wbi_oe[i] not i64"))
        .collect();
    let le: String = oe
        .iter()
        .filter(|x| **x < ae.len() as i64)
        .map(|x| *x as usize)
        .fold(String::new(), |acc, x| acc + &ae[x..(x + 1)]);
    let le: String = le[..32].into();
    bench.commit_state(move |s| s.insert("wbi_salt".into(), le.clone()));
    Ok(())
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

    #[tokio::test]
    async fn test_fetch_wbi_salt() -> Result<()> {
        let bench = Bench::new();
        fetch_wbi_salt(&bench).await?;
        let state = bench.state();
        println!("state: {:?}", &state);
        let salt: &String = state.get("wbi_salt").unwrap();
        println!("wbi_salt: {}", salt);
        assert_eq!(salt.len(), 32);
        Ok(())
    }
}
