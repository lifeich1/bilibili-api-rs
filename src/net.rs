pub fn new_client() -> reqwest::Result<reqwest::Client> {
    reqwest::ClientBuilder::new()
        .user_agent("Mozilla/5.0")
        .referer(false)
        .default_headers({
            let mut hdrs = reqwest::header::HeaderMap::new();
            hdrs.insert("Referer", reqwest::header::HeaderValue::from_static("https://www.bilibili.com"));
            hdrs
        })
        .build()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn real_test_1n_get_video_info() -> reqwest::Result<()> {
        let req = new_client()?
            .get("https://api.bilibili.com/x/web-interface/view")
            .query(&[("bvid", "BV1uv411q7Mv")]);
        println!("Req: {:?}", req);
        let resp = req.send().await?;
        assert_eq!(resp.status(), reqwest::StatusCode::OK);
        let j = resp.json::<serde_json::Value>().await?;
        if let serde_json::Value::Object(o) = j {
            println!("Body: {:?}\n", o);
        } else {
            println!("Body: {}\n", j);
            panic!("body is not normal json response.");
        }

        Ok(())
    }

    #[test]
    fn test_new_client() -> Result<(), reqwest::Error> {
        new_client()?;
        Ok(())
    }
}
