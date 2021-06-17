use bilibili_api_rs::{api, NetContext, Result as ApiResult};

#[tokio::test]
#[ignore]
async fn get_info() -> ApiResult<()> {
    let n = NetContext::new()?;
    let u = api::User::new(&n, 15810);
    let v = u.get_info().await?;
    assert!(!v.is_null());
    println!("info: {}", v.to_string());

    Ok(())
}
