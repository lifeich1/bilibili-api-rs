use crate::{ApiRequest, ApiResult};
use bevy::prelude::*;
use bevy::tasks::TaskPool;
use futures_lite::future;
use std::ops::Deref;
use tokio::runtime;

/// A bevy plugin for easily emit api requests as io tasks.
pub struct ApiRuntimePlugin {
    rt_hdl: runtime::Handle,
}

/// The resource carries tokio runtime handle.
pub struct RuntimeHandle(runtime::Handle);

/// Helper api for spawn api request on runtime and bevy task pool.
pub trait SpawnOnWorld {
    fn spawn_on(self, task_pool: &TaskPool, rt_hdl: Res<RuntimeHandle>) -> ApiRequestTask;
}

impl SpawnOnWorld for ApiResult<ApiRequest> {
    fn spawn_on(self, task_pool: &TaskPool, rt_hdl: Res<RuntimeHandle>) -> ApiRequestTask {
        let rt = rt_hdl.0.clone();
        ApiRequestTask(task_pool.spawn(async move { rt.block_on(self?.query()) }))
    }
}

/// Component that hold the future of api reqeust
pub struct ApiRequestTask(bevy::tasks::Task<ApiResult<serde_json::Value>>);

/// Component that hold the final api result
pub struct ApiTaskResult(ApiResult<serde_json::Value>);

impl Deref for ApiTaskResult {
    type Target = ApiResult<serde_json::Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ApiRuntimePlugin {
    pub fn new(rt: &runtime::Runtime) -> Self {
        Self {
            rt_hdl: rt.handle().clone(),
        }
    }
}

impl Plugin for ApiRuntimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(RuntimeHandle(self.rt_hdl.clone()))
            .add_system(handle_tasks.system());
    }
}

fn handle_tasks(mut commands: Commands, mut api_tasks: Query<(Entity, &mut ApiRequestTask)>) {
    for (entity, mut task) in api_tasks.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            commands
                .entity(entity)
                .remove::<ApiRequestTask>()
                .insert(ApiTaskResult(result));
        }
    }
}
