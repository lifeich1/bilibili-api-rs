use crate::{ApiRequest, ApiResult};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use futures_lite::future;
use tokio::runtime;

/// A bevy plugin for easily emit api requests as io tasks.
pub struct ApiRuntimePlugin {
    ctx: crate::Context,
    rt_hdl: runtime::Handle,
}

/// The resource carries tokio runtime handle.
pub struct RuntimeHandle(runtime::Handle);

#[derive(Clone)]
pub enum ApiRequestTag {
    Num(i64),
    UNum(u64),
    Str(String),
}

/// Event that emit api request
pub struct ApiRequestEvent {
    req: ApiResult<ApiRequest>,
    tag: ApiRequestTag,
}

/// Event that report the api result
pub struct ApiTaskResultEvent {
    result: ApiResult<serde_json::Value>,
    tag: ApiRequestTag,
}

/// Component that hold the future of api reqeust
pub struct ApiRequestTask(bevy::tasks::Task<ApiResult<serde_json::Value>>);

impl ApiRuntimePlugin {
    pub fn new(ctx: &crate::Context, rt: &runtime::Runtime) -> Self {
        Self {
            ctx: ctx.clone(),
            rt_hdl: rt.handle().clone(),
        }
    }
}

impl Plugin for ApiRuntimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(RuntimeHandle(self.rt_hdl.clone()))
            .insert_resource(self.ctx.clone())
            .add_event::<ApiRequestEvent>()
            .add_event::<ApiTaskResultEvent>()
            .add_system(handle_tasks.system());
    }
}

fn emit_tasks(
    mut commands: Commands,
    mut emit_chan: EventReader<ApiRequestEvent>,
    mut result_chan: EventWriter<ApiTaskResultEvent>,
    runtime: Res<RuntimeHandle>,
    task_pool: Res<IoTaskPool>,
) {
    for ev in emit_chan.iter() {
        match &ev.req {
            Ok(request) => {
                let req = request.clone();
                let rt = runtime.0.clone();
                let task = ApiRequestTask(task_pool.spawn(async move { rt.block_on(req.query()) }));
                commands.spawn().insert(ev.tag.clone()).insert(task);
            }
            Err(e) => result_chan.send(ApiTaskResultEvent {
                result: Err(e),
                tag: ev.tag.clone(),
            })
        }
    }
}

fn handle_tasks(
    mut commands: Commands,
    mut api_tasks: Query<(Entity, &ApiRequestTag, &mut ApiRequestTask)>,
    mut result_chan: EventWriter<ApiTaskResultEvent>,
) {
    for (entity, tag, mut task) in api_tasks.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            commands
                .entity(entity)
                .remove::<ApiRequestTask>()
                .despawn();

            result_chan.send(ApiTaskResultEvent {
                result,
                tag: tag.clone(),
            });
        }
    }
}
