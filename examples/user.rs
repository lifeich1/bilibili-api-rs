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
    let cli = Client::new();
    let wuyi = cli.user(1472906636);
    let info = wuyi.info().await?;
    println!("wuyi info: {}", info);
    let latest = wuyi.latest_videos().await?;
    println!("wuyi latest_videos: {}", latest);
    Ok(())
}
