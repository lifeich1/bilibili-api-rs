use bilibili_api_rs::{api, NetContext, Result as ApiResult};

#[test]
fn get_info() -> ApiResult<()> {
    let n = NetContext::new();
    let u = api::User::new(&n, "9999999");
    let v = u.get_info();
    assert!(!v.is_null());
    println!("info: {:?}", v);

    Ok(())
}
