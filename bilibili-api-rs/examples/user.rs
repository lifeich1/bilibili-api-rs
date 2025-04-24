use anyhow::{bail, Result};
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
    let mk33 = 210_628;
    let mut init_cnt = 0;
    let info = loop {
        if let Ok(v) = cli.user(mk33).info().await {
            break v;
        }
        init_cnt += 1;
        if init_cnt > 5 {
            bail!("init retry too many: {init_cnt}");
        }
    };
    println!("retried {init_cnt} for initialization.");
    println!("mk33 info: {info}");
    let latest = cli.user(mk33).latest_videos().await?;
    println!("mk33 latest_videos: {latest}");
    let latest = cli.user(mk33).recent_posts().await?;
    println!("mk33 recent_posts: {latest}");
    let maybe_not_found = cli.user(mk33).live_info().await;
    println!("mk33 live_info: {maybe_not_found:?}");
    cli.user(mk33).room_id(5229);
    let latest = cli.user(mk33).live_info().await?;
    println!("mk33 live_info: {latest}");
    Ok(())
}
