use bilibili_api_rs::api::{self, xlive};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = api::Context::new()?;
    let x = ctx.xlive();

    let v = x
        .get_list(
            xlive::PAreaID::Vup,
            xlive::AreaID::All,
            xlive::ListSortType::Entropy,
            1,
        )?
        .query()
        .await?;
    let l = v["list"].as_array().unwrap();
    for i in l.iter() {
        println!(
            "{} {} {}",
            i["uname"].as_str().unwrap(),
            i["online"].as_i64().unwrap(),
            i["title"].as_str().unwrap()
        );
    }

    Ok(())
}
