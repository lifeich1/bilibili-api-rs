// TODO encwbi
// TODO generic do request
use super::{Bench, Lodash};
use anyhow::{bail, Result};
use log::{debug, trace};
use regex::Regex;
use reqwest::header::{COOKIE, REFERER, USER_AGENT};
use serde_json::json;
use std::collections::btree_map::BTreeMap;

type Json = serde_json::Value;

async fn do_api_req(bench: &Bench, api_path: Vec<&str>, opts: Json) -> Result<Json> {
    api_result_validate(do_req_twice(bench, api_path, opts).await?)
}

fn api_result_validate(mut resp: Json) -> Result<Json> {
    if matches!(resp["code"].as_i64(), Some(0)) {
        Ok(resp["data"].take())
    } else {
        bail!(
            "bilibili api reject: {} {}",
            resp["code"].as_i64().unwrap_or(-1),
            resp["message"].as_str().unwrap_or("unknown")
        );
    }
}

async fn do_req_twice(bench: &Bench, api_path: Vec<&str>, opts: Json) -> Result<Json> {
    let state = bench.state();
    if state.get("wbi_salt").is_none() {
        debug!("do_req init salt");
        fetch_wbi_salt(bench).await?;
    }
    debug!("do_req once");
    if let Ok(res) = do_req(bench, api_path.clone(), opts.clone()).await {
        debug!("do_req once done");
        return Ok(res);
    }
    debug!("do_req twice: fetch_wbi_salt");
    fetch_wbi_salt(bench).await?;
    debug!("do_req twice");
    do_req(bench, api_path, opts).await
}

fn gen_cookie(bench: &Bench) -> String {
    let data = bench.data();
    data["cookies"]
        .as_object()
        .expect("data[cookies] should be map")
        .iter()
        .map(|t| {
            format!(
                "{}={}",
                t.0,
                t.1.as_str().expect("cookie value should be str")
            )
        })
        .fold(String::new(), |acc, s| {
            if acc.len() > 0 {
                format!("{}; {}", acc, s)
            } else {
                s
            }
        })
}

async fn do_req(bench: &Bench, api_path: Vec<&str>, mut opts: Json) -> Result<Json> {
    let data = bench.data();
    let cli = reqwest::Client::new();
    let api = data["api"].got(api_path);
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
            COOKIE,
            gen_cookie(bench)
                .parse::<reqwest::header::HeaderValue>()
                .unwrap(),
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
    trace!("request sending: {:?}", &req);
    Ok(serde_json::from_str(&req.send().await?.text().await?)?)
}

fn enc_wbi(bench: &Bench, mut opts: Json, ts: i64) -> Json {
    let mut qs: BTreeMap<&str, String> = BTreeMap::new();
    qs.insert("wts", ts.to_string());
    for (k, v) in opts["query"].as_object().expect("query not json object") {
        qs.insert(
            k,
            if v.is_string() {
                v.as_str().unwrap().to_owned()
            } else {
                serde_json::to_string(v).expect("query value to_string error")
            },
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
    let le = wbi_salt_compute(bench, imgurl, suburl);
    bench.commit_state(move |s| s.insert("wbi_salt".into(), le.clone()));
    Ok(())
}

fn wbi_parse_ae(imgurl: &str, suburl: &str) -> Option<String> {
    let Ok(re) = Regex::new(r"https://i0\.hdslb\.com/bfs/wbi/(\w+)\.png") else {
        return None;
    };
    let img = re.captures(imgurl)?.get(1)?.as_str();
    let sub = re.captures(suburl)?.get(1)?.as_str();
    Some(img.to_owned() + sub)
}

fn wbi_salt_compute(bench: &Bench, imgurl: &str, suburl: &str) -> String {
    let ae: String = match wbi_parse_ae(imgurl, suburl) {
        Some(s) => s,
        None => {
            imgurl[imgurl.len() - 36..imgurl.len() - 4].to_owned()
                + &suburl[suburl.len() - 36..suburl.len() - 4]
        }
    };
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
    le[..32].into()
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
        do_api_req(
            &self.0,
            vec!["user", "info", "info"],
            json!({"query":{
                "mid":self.1,
                "token": "",
                "platform": "web",
                "web_location": 1550101,
            }}),
        )
        .await
    }

    pub async fn latest_videos(&self) -> Result<Json> {
        do_api_req(
            &self.0,
            vec!["user", "info", "video"],
            json!({
                "query": {
                    "mid": self.1,
                    "ps": 30, "tid": 0, "pn": 1,
                    "order": "pubdate",
                    "keyword": "",
                }
            }),
        )
        .await
    }
}

impl Xlive {
    pub async fn list(&self, pn: i64) -> Result<Json> {
        do_api_req(
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

    #[test]
    fn test_wbi_salt_compute() {
        let bench = Bench::new();
        let le = wbi_salt_compute(
            &bench,
            "https://i0.hdslb.com/bfs/wbi/e130e5f398924e569b7cca9f4713ec63.png",
            "https://i0.hdslb.com/bfs/wbi/65c711c1f26b475a9305dad9f9903782.png",
        );
        assert_eq!(le, "5a73a9f6609390773b53586cce514c2e");
    }

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

    #[test]
    fn test_enc_wbi2() {
        let salt = "5a73a9f6609390773b53586cce514c2e";
        let bench = Bench::new();
        bench.commit_state(|s| s.insert("wbi_salt".into(), salt.to_owned()));
        let opts = enc_wbi(
            &bench,
            json!({
                "query": {
                    "mid": 1472906636,
                    "token": "",
                    "platform": "web",
                    "web_location": 1550101,
                }
            }),
            1686230003,
        );
        assert_eq!(
            opts,
            json!({
                "_uq": "mid=1472906636&platform=web&token=&web_location=1550101&wts=1686230003",
                "query": {
                    "wts": 1686230003,
                    "w_rid": "9946c05f7b3d5a8505a97e1b8daab2be",
                    "mid": 1472906636,
                    "token": "",
                    "platform": "web",
                    "web_location": 1550101,
                },
            })
        );
    }
}
