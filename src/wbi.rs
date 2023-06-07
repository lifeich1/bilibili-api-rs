// TODO encwbi
// TODO generic do request
use super::Bench;
use anyhow::{bail, Result};
use reqwest::header::{REFERER, USER_AGENT};
use serde_json::json;
use std::collections::btree_map::BTreeMap;

type Json = serde_json::Value;

async fn do_req_twice(bench: &Bench, api_path: Vec<&str>, opts: Json) -> Result<Json> {
    let state = bench.state();
    if state.get("wbi_salt").is_none() {
        fetch_wbi_salt(bench).await?;
    }
    if let Ok(res) = do_req(bench, api_path.clone(), opts.clone()).await {
        return Ok(res);
    }
    fetch_wbi_salt(bench).await?;
    do_req(bench, api_path, opts).await
}

async fn do_req(bench: &Bench, api_path: Vec<&str>, mut opts: Json) -> Result<Json> {
    let data = bench.data();
    let cli = reqwest::Client::new();
    let mut api = &data["api"];
    for p in api_path {
        api = &api[p];
    }
    if api["wbi"].as_bool().unwrap_or(false) {
        let ts = chrono::Local::now().timestamp();
        opts = enc_wbi(bench, opts, ts);
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

fn enc_wbi(bench: &Bench, mut opts: Json, ts: i64) -> Json {
    let mut qs: BTreeMap<&str, String> = BTreeMap::new();
    qs.insert("wts", ts.to_string());
    for (k, v) in opts["query"].as_object().expect("query not json object") {
        qs.insert(
            k,
            serde_json::to_string(v).expect("query value to_string error"),
        );
    }
    let uq: String = qs
        .iter()
        .map(|t| format!("{}={}", t.0, t.1))
        .fold(String::new(), |acc, q| {
            if acc.len() > 0 {
                acc + "&" + &q
            } else {
                q
            }
        });
    opts["_uq"] = uq.clone().into();
    opts["query"]["wts"] = ts.into();
    let state = bench.state();
    opts["query"]["w_rid"] = Json::String(format!(
        "{:x}",
        md5::compute(
            uq + state
                .get("wbi_salt".into())
                .expect("salt should be prepared before enc_wbi")
        )
    ));
    opts
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

#[derive(Clone, Debug)]
pub struct Client {
    bench_: Bench,
}

#[derive(Clone, Debug)]
pub struct User(Bench, i64);

#[derive(Clone, Debug)]
pub struct Xlive(Bench, i64, i64);

impl Client {
    pub fn new() -> Self {
        Self {
            bench_: Bench::new(),
        }
    }

    pub fn user(&self, mid: i64) -> User {
        User(self.bench_.clone(), mid)
    }

    pub fn xlive(&self, area: i64, sub: i64) -> Xlive {
        Xlive(self.bench_.clone(), area, sub)
    }
}

impl User {
    pub async fn info(&self) -> Result<Json> {
        do_req_twice(
            &self.0,
            vec!["user", "info", "info"],
            json!({"query":{"mid":self.1}}),
        )
        .await
    }

    pub async fn latest_videos(&self) -> Result<Json> {
        do_req_twice(
            &self.0,
            vec!["user", "info", "video"],
            json!({
                "query": {
                    "mid": self.1,
                    "ps": 30, "tid": 0, "pn": 1,
                    "order": "pubdate",
                }
            }),
        )
        .await
    }
}

impl Xlive {
    pub async fn list(&self, pn: i64) -> Result<Json> {
        do_req_twice(
            &self.0,
            vec!["xlive", "info", "get_list"],
            json!({
                "query": {
                    "parent_area_id": self.1,
                    "area_id": self.2,
                    "page": pn,
                    "sort_type": "sort_type_291",
                    "platform": "web",
                }
            }),
        )
        .await
    }
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

    #[test]
    fn test_enc_wbi() {
        let salt = "b7ot4is0ba.3cp9fi5:ce0eme/l9d84s";
        let bench = Bench::new();
        bench.commit_state(|s| s.insert("wbi_salt".into(), salt.to_owned()));
        let opts = enc_wbi(
            &bench,
            json!({
                "query": {
                    "mid": 213741,
                }
            }),
            1686163791,
        );
        assert_eq!(
            opts,
            json!({
                "_uq": "mid=213741&wts=1686163791",
                "query": {
                    "w_rid": "dc7bb638dc082c354fd9624b72374f3b",
                    "mid": 213741,
                    "wts": 1686163791,
                },
            })
        );
    }
}
