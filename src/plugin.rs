use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;
use bevy::tasks::IoTaskPool;
use tokio::runtime;
use futures_lite::future;
use crate::{ApiRequest, ApiResult};

/// A bevy plugin for easily emit api requests as io tasks.
pub struct ApiRuntimePlugin {
    rt_hdl: runtime::Handle;
}

pub struct RuntimeHandle(runtime::Handle);

pub struct ApiRequestTodo(ApiResult<ApiRequest>);

struct ApiRequestTask(bevy::tasks::Task<ApiResult<serde_json::Value>>);

pub struct ApiTaskResult(ApiResult<serde_json::Value>);

impl ApiRuntimePlugin {
    pub fn new(rt: &runtime::Runtime) -> Self {
        Self {
            rt_hdl: rt.handle().clone(),
        }
    }
}

impl Plugin for ApiRuntimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(RuntimeHandle(self.rt.handle().clone()))
            .add_system(emit_tasks.system());
            .add_system(handle_tasks.system());
    }
}

fn emit_tasks(
    mut commands: Commands,
    mut todos: Query<(Entity, ApiRequestTodo)>,
    thread_pool: &Res<IoTaskPool>,
    rt_handle: &Res<RuntimeHandle>,
    ) {
    for (entity, todo) in todos.iter() {
        match todo {
            Ok(req) => {
                let task = thread_pool.spwan(rt_handle.0.spwan(req.query()));
                commands.entity(entity).insert(ApiRequestTask(task));
            },
            Err(e) => {
                commands.entity(entity).insert(ApiTaskResult(Err(e)));
            },
        }
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
