use bevy::{app::AppExit, prelude::*};
use bilibili_api_rs::{
    api,
    plugin::{ApiRequestEvent, ApiRuntimePlugin, ApiTaskResultEvent},
};
use chrono::naive::NaiveDateTime;
use tokio::runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = runtime::Runtime::new()?;
    let ctx = api::Context::new()?;

    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(ApiRuntimePlugin::new(&ctx, &rt))
        .add_startup_system(emit_request.system())
        .add_system(handle_result.system())
        .run();

    Ok(())
}

const UID: u64 = 10592068;

fn emit_request(mut req_chan: EventWriter<ApiRequestEvent>, context: Res<api::Context>) {
    req_chan.send(ApiRequestEvent {
        req: context.new_user(UID).video_list(1),
        tag: String::from("Thank you, Shenme-dioShio!").into(),
    });
}

fn handle_result(
    mut result_chan: EventReader<ApiTaskResultEvent>,
    mut exit_chan: EventWriter<AppExit>,
) {
    for ev in result_chan.iter() {
        match ev.result.as_ref() {
            Ok(v) => print_vid_list(v),
            Err(e) => println!("print error: {}", e),
        }
        println!("tag: {}", ev.tag.to_string());
        exit_chan.send(AppExit {});
    }
}

fn print_vid_list(result: &serde_json::Value) {
    match result["list"]["vlist"].as_array() {
        Some(list) => {
            for vid in list.iter() {
                let bv = vid["bvid"].as_str().unwrap_or("no-bvid");
                let title = vid["title"].as_str().unwrap_or("no-title");
                let ts = vid["created"].as_i64().unwrap_or(0);
                let ts = NaiveDateTime::from_timestamp(ts, 0)
                    .format("%Y-%m-%d %H:%M")
                    .to_string();
                println!("{} {} {}", ts, bv, title);
            }
        }
        None => println!("video list empty!!"),
    }
}
