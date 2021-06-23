use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;
use bevy::tasks::IoTaskPool;
use tokio::runtime;
use futures_lite::future;
use crate::{ApiRequest, ApiResult};

/// A bevy plugin for easily emit api requests as io tasks.
pub struct ApiRuntimePlugin {
    rt: runtime::Runtime,
}

pub struct RuntimeHandle(runtime::Handle);

pub struct ApiRequestTask(bevy::tasks::Task<ApiResult<serde_json::Value>>);

pub struct ApiTaskResult(ApiResult<serde_json::Value>);

impl ApiRuntimePlugin {
    pub fn new() -> Self {
        Self {
            rt: runtime::Runtime::new()
                .expect("System failed to create multi-thread tokio runtime")
        }
    }

    pub fn spwan(commands: &mut EntityCommands,
                 thread_pool: &Res<IoTaskPool>,
                 rt_handle: &Res<RuntimeHandle>,
                 req: ApiResult<ApiRequest>,
                 ) {
        let rt = rt_handle.0.clone();
        let task = thread_pool.spwan(async move {
            rt.block_on(async move {
                req?.await
            });
        });
        commands.insert(ApiRequestTask(task));
    }
}

impl Plugin for ApiRuntimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(RuntimeHandle(self.rt.handle().clone()))
            .add_system(handle_tasks.system());
    }
}

fn handle_tasks(
    mut commands: Commands,
    mut api_tasks: Query<(Entity, &mut ApiRequestTask)>,
    ) {
    for (entity, mut task) in api_tasks.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut *task)) {
            commands.entity(entity)
                .remove::<ApiRequestTask>()
                .insert(ApiTaskResult(result));
        }
    }
}
