use bilibili_api_rs::{ApiResult, Context};
use simple_logger::SimpleLogger;

//#[ignore]
#[tokio::test]
async fn user_api_async_test() -> ApiResult<()> {
    SimpleLogger::new().with_utc_timestamps().init().unwrap();

    let n = Context::new()?;
    let u = n.new_user(15810);
    let v = u.get_info()?.query().await?;

    assert!(!v.is_null());
    println!("info: {}", v.to_string());

    assert_eq!(v["mid"].as_i64().unwrap(), 15810);
    assert_eq!(v["name"].as_str().unwrap(), "Mr.Quin");

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
