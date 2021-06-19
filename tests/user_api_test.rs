use bilibili_api_rs::{Context, Result as ApiResult};

#[tokio::test]
#[ignore]
async fn user_api_test() -> ApiResult<()> {
    let n = Context::new()?;
    let u = n.new_user(15810);
    let v = u.get_info().await?;
    assert!(!v.is_null());
    println!("info: {}", v.to_string());

    assert_eq!(v["mid"].as_i64().unwrap(), 15810);
    assert_eq!(v["name"].as_str().unwrap(), "Mr.Quin");

    Ok(())
}
