use anyhow::Result;
use bilibili_api_rs::Client;

#[tokio::main]
async fn main() -> Result<()> {
    stderrlog::new()
        .module("bilibili_api_rs")
        .module("reqwest")
        .verbosity(stderrlog::LogLevelNum::Trace)
        .init()
        .unwrap();
    let mut cli = Client::new();
    let wuyi = 1_472_906_636;
    let info = cli.user(wuyi).info().await?;
    println!("wuyi info: {info}");
    let latest = cli.user(wuyi).latest_videos().await?;
    println!("wuyi latest_videos: {latest}");
    let latest = cli.user(wuyi).recent_posts().await?;
    println!("wuyi recent_posts: {latest}");
    Ok(())
}
