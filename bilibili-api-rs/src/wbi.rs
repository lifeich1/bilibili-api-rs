//! WBI means "web bibilili interface".
use super::{cred_utils, Bench, Lodash, StateData};
use anyhow::{bail, Context, Result};
use log::{debug, trace};
use regex::Regex;
use reqwest::header::{CONTENT_TYPE, COOKIE, REFERER, USER_AGENT};
use serde_json::json;
use std::collections::btree_map::BTreeMap;
use std::collections::HashSet;
use tokio::sync::mpsc;

type Json = serde_json::Value;

async fn do_api_req(bench: &Bench, api_path: Json, opts: Json) -> Result<Json> {
    api_result_validate(do_req_twice(bench, api_path, opts).await?)
}

fn api_result_validate(mut resp: Json) -> Result<Json> {
    if matches!(resp, Json::String(_)) {
        trace!("plaintext response bypass api_result_validate");
        return Ok(resp);
    }
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

async fn do_req_twice(bench: &Bench, api_path: Json, opts: Json) -> Result<Json> {
    let state = &bench.state;
    let mut mut_stat: Option<StateData> = None;

    let k_domain = "Domain";
    if state.get(k_domain).is_none() {
        mut_stat = mut_stat
            .or_else(|| Some(state.clone()))
            .map(|s| s.update(k_domain.into(), ".bilibili.com".into()));
    }

    let k_salt = "wbi_salt";
    if state.get(k_salt).is_none() {
        debug!("do_req init salt");
        let salt = fetch_wbi_salt(bench).await?;
        mut_stat = mut_stat
            .or_else(|| Some(state.clone()))
            .map(|s| s.update(k_salt.into(), salt));
    }

    let k_uvid = "buvid3";
    if state.get(k_uvid).is_none() {
        debug!("do_req init uvid");
        let uvid = fetch_uvid(bench).await?;
        mut_stat = mut_stat
            .or_else(|| Some(state.clone()))
            .map(|s| s.update(k_uvid.into(), uvid));
    }

    if let Some(new_stat) = mut_stat {
        bench.update_state(new_stat);
        bail!("Require retry for update state");
    }

    do_req(bench, api_path, opts).await
}

fn gen_cookie(bench: &Bench) -> String {
    let data = &bench.data;
    let state = &bench.state;
    data["cookie_state"]
        .as_array()
        .expect("data[cookie_state] should be array")
        .iter()
        .map(|x| x.as_str().expect("item of 'cookie_state' should be string"))
        .map(|k| (k, state.get(k)))
        .filter_map(|p| p.1.map(|v| (p.0, v)))
        .map(|p| format!("{}={}", p.0, p.1))
        .fold(String::new(), |acc, s| {
            if acc.is_empty() {
                s
            } else {
                format!("{acc}; {s}")
            }
        })
}

trait AttachHeaders {
    fn headers_of_bench(self, bench: &Bench) -> Self;
}

impl AttachHeaders for reqwest::RequestBuilder {
    fn headers_of_bench(self, bench: &Bench) -> Self {
        let data = &bench.data;
        self.header(
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
    }
}

async fn do_req(bench: &Bench, api_path: Json, mut opts: Json) -> Result<Json> {
    debug!("do_req api_path: {:?}", &api_path);
    let data = &bench.data;
    let cli = reqwest::Client::new();
    let api = data["api"].at(api_path);
    if api["wbi"].as_bool().unwrap_or(false) {
        let ts = chrono::Local::now().timestamp();
        opts = enc_wbi(bench, opts, ts);
    }
    let req = cli
        .request(
            api["method"].as_str().unwrap_or("GET").parse().unwrap(),
            api["url"].as_str().unwrap(),
        )
        .headers_of_bench(bench)
        .query(&opts["query"]);
    trace!("do_req: {:?}", &req);
    if api["raw_content"].as_bool().unwrap_or(false) {
        return Ok(Json::String(req.send().await?.text().await?));
    }
    Ok(
        serde_json::from_str(&req.send().await?.text().await?).map(|resp| {
            trace!("do_req resp: {:?}", &resp);
            resp
        })?,
    )
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
            if acc.is_empty() {
                q
            } else {
                acc + "&" + &q
            }
        });
    opts["_uq"] = uq.clone().into();
    opts["query"]["wts"] = ts.into();
    let state = &bench.state;
    opts["query"]["w_rid"] = Json::String(format!(
        "{:x}",
        md5::compute(
            uq + state
                .get("wbi_salt")
                .expect("salt should be prepared before enc_wbi")
        )
    ));
    opts
}

async fn fetch_uvid(bench: &Bench) -> Result<String> {
    let mut spi = do_req(bench, json!(["credential", "info", "spi"]), json!({})).await?;
    let Json::String(uvid) = spi["data"]["b_3"].take() else {
        bail!("fetch_uvid: b_3 invalid");
    };
    let Json::String(uvid4) = spi["data"]["b_4"].take() else {
        bail!("fetch_uvid: b_4 invalid");
    };
    active_buvid(bench, &uvid, &uvid4).await?;
    Ok(uvid)
}

async fn active_buvid(bench: &Bench, uvid: &str, uvid4: &str) -> Result<()> {
    let active_id = format!(
        "{}{:05}infoc",
        uuid::Uuid::new_v4().hyphenated(),
        chrono::Local::now().timestamp_subsec_nanos() % 100_000
    );
    let payload = cred_utils::gen_payload(&active_id);
    let cli = reqwest::Client::new();
    let api = &bench.data["api"]["credential"]["operate"]["active"];
    let mut buvid_bench = bench.clone();
    {
        let cookie = &mut buvid_bench.state;
        cookie.insert("buvid3".into(), uvid.into());
        cookie.insert("buvid4".into(), uvid4.into());
        cookie.insert("buvid_fp".into(), cred_utils::gen_buvid_fp(&payload)?);
        cookie.insert("_uuid".into(), active_id);
    }
    let req = cli
        .request(
            api["method"].as_str().unwrap_or("GET").parse().unwrap(),
            api["url"].as_str().unwrap(),
        )
        .headers_of_bench(&buvid_bench)
        .header(
            CONTENT_TYPE,
            "application/json"
                .parse::<reqwest::header::HeaderValue>()
                .unwrap(),
        )
        .json(&payload);
    trace!("active buvid {:?}", &req);
    let resp: serde_json::Value = serde_json::from_str(&req.send().await?.text().await?)?;
    trace!("active buvid resp: {:?}", &resp);
    let code = &resp["code"];
    if !matches!(code.as_i64(), Some(0)) {
        bail!(
            "active buvid failed, code {:?}, msg {:?}",
            code,
            resp["msg"]
        );
    }
    Ok(())
}

async fn fetch_wbi_salt(bench: &Bench) -> Result<String> {
    let nav = do_req(bench, json!(["credential", "info", "valid"]), json!({})).await?;
    let Some(imgurl) = nav["data"]["wbi_img"]["img_url"].as_str() else {
        bail!("fetch_wbi_salt: wbi_img/img_url invalid");
    };
    let Some(suburl) = nav["data"]["wbi_img"]["sub_url"].as_str() else {
        bail!("fetch_wbi_salt: wbi_img/sub_url invalid");
    };
    Ok(wbi_salt_compute(bench, imgurl, suburl))
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
    let ae: String = wbi_parse_ae(imgurl, suburl).unwrap_or_else(|| {
        imgurl[imgurl.len() - 36..imgurl.len() - 4].to_owned()
            + &suburl[suburl.len() - 36..suburl.len() - 4]
    });
    let oe: Vec<i64> = bench.data["wbi_oe"]
        .as_array()
        .expect("wbi_oe not array")
        .iter()
        .map(|v| v.as_i64().expect("wbi_oe[i] not i64"))
        .collect();
    let le: String = oe
        .iter()
        .filter_map(|x| usize::try_from(*x).ok())
        .filter(|x| *x < ae.len())
        .fold(String::new(), |acc, x| acc + &ae[x..=x]);
    le[..32].into()
}

/// The root client base, see also [TOP][crate].
#[derive(Debug)]
pub struct Client {
    bench: Bench,
    rx: mpsc::Receiver<StateData>,
}

/// Remember user id and do GETs.
#[derive(Clone, Debug)]
pub struct User(Bench, i64);

/// Remember parent area id and sub area id then do GETs.
#[derive(Clone, Debug)]
pub struct Xlive(Bench, i64, i64);

impl Client {
    /// Create a default instance.
    #[must_use]
    pub fn new() -> Self {
        let (bench, rx) = Bench::new();
        Self { bench, rx }
    }

    /// `mid` is *uid*
    #[must_use]
    pub fn user(&mut self, mid: i64) -> User {
        self.do_sync();
        User(self.bench.clone(), mid)
    }

    /// Renaming for logical. `area` is *`parent_area_id`*, `sub` is *`area_id`*.
    #[must_use]
    pub fn xlive(&mut self, area: i64, sub: i64) -> Xlive {
        self.do_sync();
        Xlive(self.bench.clone(), area, sub)
    }

    fn do_sync(&mut self) {
        match self.rx.try_recv() {
            Ok(s) => {
                trace!("current state: {:?}", &s);
                self.bench.state = s;
            }
            Err(mpsc::error::TryRecvError::Disconnected) => {
                panic!("existing client should have health channel")
            }
            _ => (),
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl User {
    /// See also [*api_info/user:info/info*][api_info/user]
    ///
    /// [api_info/user]: https://github.com/lifeich1/bilibili-api-rs/blob/master/src/api_info/user.json
    ///
    /// # Errors
    /// Throw network errors or api errors.
    #[deprecated = "WBI is stateful protected. Replace it with card/live_info"]
    pub async fn info(&self) -> Result<Json> {
        do_api_req(
            &self.0,
            json!(["user", "info", "info"]),
            json!({"query":{
                "mid":self.1,
                "web_location": 1_550_101,
            }}),
        )
        .await
    }

    /// See also [*api_info/unstable:videos*][api_info/unstable]
    ///
    /// [api_info/unstable]: https://github.com/lifeich1/bilibili-api-rs/blob/master/src/api_info/unstable.json
    ///
    /// # Errors
    /// Throw network errors or api errors.
    pub async fn latest_videos(&self) -> Result<Json> {
        do_api_req(
            &self.0,
            json!(["unstable", "videos"]),
            json!({
                "query": {
                    "mobi_app": "web",
                    "type": 1,
                    "biz_id": self.1,
                    "oid": "",
                    "otype": 2,
                    "ps": 2,
                    "direction": false,
                    "desc": true,
                    "sort_field": 1,
                    "tid": 0,
                    "with_current": false
                }
            }),
        )
        .await
    }

    /// See also [*api_info/user:info/dynamic*][api_info/user]
    ///
    /// [api_info/user]: https://github.com/lifeich1/bilibili-api-rs/blob/master/src/api_info/user.json
    ///
    /// # Errors
    /// Throw network errors or api errors.
    pub async fn recent_posts(&self) -> Result<Json> {
        do_api_req(
            &self.0,
            json!(["user", "info", "dynamic"]),
            json!({
                "query": {
                    "host_uid": self.1,
                    "offset_dynamic_id": 0,
                    "need_top": 0,
                }
            }),
        )
        .await
    }

    /// See also [*api_info/unstable:card*][api_info/unstable]
    ///
    /// [api_info/unstable]: https://github.com/lifeich1/bilibili-api-rs/blob/master/src/api_info/unstable.json
    ///
    /// # Errors
    /// Throw network errors or api errors.
    pub async fn card(&self) -> Result<Json> {
        do_api_req(
            &self.0,
            json!(["unstable", "card"]),
            json!({
                "query": {
                    "mid": self.1,
                    "photo": 1,
                }
            }),
        )
        .await
    }

    /// Invoke `search_room` if room id not found, otherwise query room info.
    /// See also [*api_info/live:info/room_info*][api_info/live]
    ///
    /// [api_info/live]: https://github.com/lifeich1/bilibili-api-rs/blob/master/src/api_info/live.json
    ///
    /// # Errors
    /// Throw network errors or api errors.
    pub async fn live_info(&self) -> Result<Json> {
        let Some(room_id) = self.0.get_room_id(self.1) else {
            let room_id = self.search_room().await?;
            self.0.set_room_id(self.1, &room_id);
            bail!("init room id of uid {}: {}", self.1, room_id);
        };
        do_api_req(
            &self.0,
            json!(["live", "info", "room_info"]),
            json!({
                "query": {
                    "room_id": room_id,
                }
            }),
        )
        .await
    }

    /// External init room id for user.
    pub fn room_id(&self, id: i64) {
        self.0.set_room_id(self.1, &id);
    }

    /// Search room of user and filter check with room play info.
    /// See also [*api_info/unstable:room_search*][api_info/unstable]
    ///
    /// [api_info/unstable]: https://github.com/lifeich1/bilibili-api-rs/blob/master/src/api_info/unstable.json
    ///
    /// # Errors
    /// Mostly if live stream stopped. Otherwise network errors or api errors.
    ///
    /// # Panics
    /// Internal failures.
    pub async fn search_room(&self) -> Result<String> {
        let card = self.card().await?;
        let html = do_api_req(
            &self.0,
            json!(["unstable", "room_search"]),
            json!({
                "query": {
                    "keyword": card["card"]["name"],
                    "from_source": "webtop_search",
                    "spm_id_from": "333.999",
                    "search_source": 5
                }
            }),
        )
        .await
        .context("api room_search")?;
        trace!("search room html: {html:?}");
        let html_txt = html
            .as_str()
            .expect("api_info/unstable:room_search result must be plaintext");
        let re = Regex::new(r#"href="//live\.bilibili\.com/(\d+)\?live_from"#)
            .expect("search room html regex must ok");
        let mut rid_set: HashSet<String> = HashSet::new();
        for (_, [room_id]) in re.captures_iter(html_txt).map(|c| c.extract()) {
            rid_set.insert(room_id.to_owned());
        }
        for room_id in rid_set {
            if let Ok(check) = do_api_req(
                &self.0,
                json!(["live", "info", "room_play_info"]),
                json!({
                    "query": {
                        "room_id": room_id,
                    }
                }),
            )
            .await
            {
                if matches!(check["uid"].as_i64(), Some(id) if id == self.1) {
                    return Ok(room_id.clone());
                }
            };
        }
        bail!("live room not found, mostly live closed, uid:{}", self.1)
    }
}

impl Xlive {
    /// Check [*api_info/xlive:info/get_list*][api_info/xlive]
    ///
    /// [api_info/xlive]: https://github.com/lifeich1/bilibili-api-rs/blob/master/src/api_info/xlive.json
    ///
    /// # Errors
    /// Throw network errors or api errors.
    pub async fn list(&self, pn: i64) -> Result<Json> {
        do_api_req(
            &self.0,
            json!(["xlive", "info", "get_list"]),
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

    fn init() {
        env_logger::builder()
            .is_test(true)
            .format_timestamp(Some(env_logger::fmt::TimestampPrecision::Micros))
            .try_init()
            .ok();
    }

    #[tokio::test]
    async fn test_cover_all_api() {
        init();
        let banned = 328_575_117;
        let cctv = 222_103_174;
        let mut cli = Client::new();
        let banned_info = cli.user(banned).card().await;
        assert!(banned_info.is_err());
        assert!(banned_info
            .unwrap_err()
            .to_string()
            .contains("Require retry for update state"));
        let banned_info = cli.user(banned).card().await;
        assert!(banned_info.is_ok());
        println!("banned_info: {banned_info:?}");
        assert!(matches!(
            banned_info.unwrap()["card"]["spacesta"].as_i64(),
            Some(-2)
        ));
        let info = cli.user(cctv).card().await;
        assert!(info.is_ok());
        assert!(cli.user(cctv).recent_posts().await.is_ok());
        assert!(cli.user(cctv).latest_videos().await.is_ok());

        let study24h = 3_546_660_198_156_783;
        let info = cli.user(study24h).search_room().await;
        println!("info: {:?}", &info);
        assert!(info.is_ok());
        assert_eq!(info.ok(), Some("32458377".to_owned()));

        let info = cli.user(study24h).live_info().await;
        assert!(info.is_err());
        assert!(info
            .unwrap_err()
            .to_string()
            .contains("init room id of uid"));

        let info = cli.user(study24h).live_info().await;
        assert!(info.is_ok());
        let info = info.unwrap();
        assert_eq!(info["room_info"]["live_status"], json!(1));
        assert_eq!(info["room_info"]["room_id"], json!(32_458_377));

        let area_drug = 1;
        let type_moe = 530;
        assert!(cli.xlive(area_drug, type_moe).list(1).await.is_ok());
    }

    #[test]
    fn test_wbi_salt_compute() {
        let bench = Bench::new().0;
        let le = wbi_salt_compute(
            &bench,
            "https://i0.hdslb.com/bfs/wbi/e130e5f398924e569b7cca9f4713ec63.png",
            "https://i0.hdslb.com/bfs/wbi/65c711c1f26b475a9305dad9f9903782.png",
        );
        assert_eq!(le, "5a73a9f6609390773b53586cce514c2e");
    }

    #[tokio::test]
    async fn test_fetch_wbi_salt() -> Result<()> {
        let bench = Bench::new().0;
        let salt = fetch_wbi_salt(&bench).await?;
        assert_eq!(salt.len(), 32);
        Ok(())
    }

    #[test]
    fn test_enc_wbi() {
        let salt = "b7ot4is0ba.3cp9fi5:ce0eme/l9d84s";
        let mut bench = Bench::new().0;
        bench.state.insert("wbi_salt".into(), salt.to_owned());
        let opts = enc_wbi(
            &bench,
            json!({
                "query": {
                    "mid": 213_741,
                }
            }),
            1_686_163_791,
        );
        assert_eq!(
            opts,
            json!({
                "_uq": "mid=213741&wts=1686163791",
                "query": {
                    "w_rid": "dc7bb638dc082c354fd9624b72374f3b",
                    "mid": 213_741,
                    "wts": 1_686_163_791,
                },
            })
        );
    }

    #[test]
    fn test_enc_wbi2() {
        let salt = "ea1db124af3c7062474693fa704f4ff8";
        let mut bench = Bench::new().0;
        bench.state.insert("wbi_salt".into(), salt.to_owned());
        let opts = enc_wbi(
            &bench,
            json!({
                "query": {
                    "mid": 222_103_174,
                    "web_location": 1_550_101,
                }
            }),
            1_714_929_805,
        );
        assert_eq!(
            opts,
            json!({
                "_uq": "mid=222103174&web_location=1550101&wts=1714929805",
                "query": {
                    "wts": 1_714_929_805,
                    "w_rid": "0ef355650a5979e017ccf135200b18f6",
                    "mid": 222_103_174,
                    "web_location": 1_550_101,
                },
            })
        );
    }
}
