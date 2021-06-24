use bevy::{app::AppExit, prelude::*, tasks::IoTaskPool};
use bilibili_api_rs::{
    api,
    plugin::{ApiRuntimePlugin, ApiTaskResult, RuntimeHandle, SpawnOnWorld},
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

struct UserInfo(String);

fn emit_request(
    mut commands: Commands,
    context: Res<api::Context>,
    thread_pool: Res<IoTaskPool>,
    runtime: Res<RuntimeHandle>,
) {
    let task = context
        .new_user(10592068)
        .video_list(1)
        .spawn_on(&thread_pool.0, runtime);
    commands
        .spawn()
        .insert(task)
        .insert(UserInfo(String::from("Thank you, Shenme-dioShio!")));
}

fn handle_result(
    mut commands: Commands,
    results: Query<(Entity, &UserInfo, &ApiTaskResult)>,
    mut exit_chan: EventWriter<AppExit>,
) {
    for (entity, info, result) in results.iter() {
        match result.as_ref() {
            Ok(v) => print_vid_list(v),
            Err(e) => println!("print error: {}", e),
        }
        println!("info: {}", info.0);
        commands.entity(entity).remove::<ApiTaskResult>();
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
