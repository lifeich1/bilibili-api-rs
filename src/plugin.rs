use crate::{error::ApiError, ApiRequest, ApiResult};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use futures_lite::future;
use std::ops::Deref;
use tokio::runtime;

/// A bevy plugin for easily emit api requests as io tasks.
pub struct ApiRuntimePlugin {
    ctx: crate::Context,
    rt_hdl: runtime::Handle,
}

/// The resource carries tokio runtime handle.
pub struct RuntimeHandle(runtime::Handle);

/// Struct tagged the api request
#[derive(Clone)]
pub struct ApiRequestTag(serde_json::Value);

impl Deref for ApiRequestTag {
    type Target = serde_json::Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for ApiRequestTag
where
    serde_json::Value: From<T>,
{
    fn from(t: T) -> Self {
        Self(serde_json::Value::from(t))
    }
}

/// Event that emit api request
pub struct ApiRequestEvent {
    pub req: ApiResult<ApiRequest>,
    pub tag: ApiRequestTag,
}

/// Event that report the api result
pub struct ApiTaskResultEvent {
    pub result: ApiResult<serde_json::Value>,
    pub tag: ApiRequestTag,
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
            .add_system(emit_tasks.system())
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
                result: Err(ApiError::general(e)),
                tag: ev.tag.clone(),
            }),
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
            commands.entity(entity).remove::<ApiRequestTask>().despawn();

            result_chan.send(ApiTaskResultEvent {
                result,
                tag: tag.clone(),
            });
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_index() {
        let uid: u64 = 10592068;
        let tag: ApiRequestTag = serde_json::json!({"uid": uid, "cmd": "refresh"}).into();
        assert_eq!(tag["uid"].as_u64().unwrap(), uid);
        assert_eq!(tag["cmd"].as_str().unwrap(), "refresh");
    }
}
