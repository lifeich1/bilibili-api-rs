use anyhow::Result;
use simple_logger::SimpleLogger;

//#[ignore]
#[tokio::test]
async fn user_api_async_test() -> Result<()> {
    println!("should begin");
    SimpleLogger::new().with_utc_timestamps().init().unwrap();

    let n = Context::new()?;
    let u = n.new_user(474593913);
    let v = u.get_info()?.query().await?;

    println!("info: {}", v.to_string());
    assert!(!v.is_null());

    assert_eq!(v["mid"].as_i64().unwrap(), 474593913);
    assert_eq!(v["name"].as_str().unwrap(), "狮子神蕾欧娜_official");

    let v = u.video_list(1)?.query().await?;
    println!("info: {}", v.to_string());
    assert!(!v.is_null());

    let v = u.stat()?.query().await?;
    println!("info: {}", v.to_string());
    assert!(!v.is_null());

    let u = n.new_user(10592068);
    let v = u.get_info()?.query().await?;
    assert_eq!(v["mid"].as_i64().unwrap(), 10592068);
    assert_eq!(v["name"].as_str().unwrap(), "RemediosShio");

    let v = u.get_info()?.query().await?;
    assert_eq!(v["mid"].as_i64().unwrap(), 10592068);
    assert_eq!(v["name"].as_str().unwrap(), "RemediosShio");

    // Should invalidate buffer, but hard to check

    let v = u.get_info()?.invalidate().query().await?;
    assert_eq!(v["mid"].as_i64().unwrap(), 10592068);
    assert_eq!(v["name"].as_str().unwrap(), "RemediosShio");

    Ok(())
}
