use actix::prelude::*;
use slog::Logger;
use serde_json;

use patoka::{
    center::send::*,
    control::message::StopTask,
    core::logger::create_logger,
    core::env,
    worker::{
        client::{WorkerClient, ClientContext},
        controller,
        plugin::{WorkerPlugin},
        processor,
        setup::*,
        task::{WorkerTask, GenTaskDefinition, TaskStatus},
        tracker::{self, *},
        worker_message::{WorkerMessage},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtaskParams {
    /// Calculate a square of `number`.
    pub number: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SubtaskExecutionResult {
    pub square: usize,
}

#[derive(Clone)]
pub struct SubtaskClient {
    log: Logger,
    ctx: ClientContext<GenTaskDefinition<SubtaskParams>>,
}

impl Actor for SubtaskClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!(self.log, "Demo Subtask Client started.");

        setup_with_controller(
            &self.ctx.task_definition.task_uuid, /* Task UUID */
            None, /* ControlMessage */
            None, /* TaskUpdate */
            ctx.address().recipient(), /* WorkerMessage */
            &self.ctx.controller_addr, /* Controller */
            self.ctx.task_definition.make_message(), /* Message to start */
            self.ctx.task_definition.name.clone(),
        );
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!(self.log, "Demo Subtask Client stopped.");

        send_center_task_finished(
            &self.ctx.task_uuid,
            TaskStatus::FinishedSuccess,
            &self.ctx.task_definition.name,
        );
    }
}

impl SubtaskClient {
    fn process_task_result(&mut self, res: SubtaskExecutionResult) {
        debug!(
            self.log,
            "Square of number {} is {:?}",
            self.ctx.task_definition.params.number,
            res.square,
        );
    }

    fn handle_worker_message(
        &mut self,
        msg: WorkerMessage,
        ctx: &mut <Self as Actor>::Context
    ) {
        trace!(self.log, "Received a worker message.");

        if let Some(task_result) = msg.payload.data.get("task_result") {
            self.process_task_result(
                serde_json::from_value(task_result.clone()).unwrap()
            );

            // Done.
            ctx.stop();
        }
    }
}

impl WorkerClient for SubtaskClient {
    type TaskDefinition = GenTaskDefinition<SubtaskParams>;

    fn new(ctx: ClientContext<Self::TaskDefinition>) -> Self {
        SubtaskClient {
            log: create_logger(&ctx.task_definition.name),
            ctx,
        }
    }
}

handler_impl_worker_message!(SubtaskClient);

handler_impl_stop_task!(SubtaskClient);

pub type Subtask = WorkerTask<SubtaskClient>;

pub fn create(
    params: SubtaskParams,
    parent_task_uuid: String,
    name: &str,
) -> processor::TaskWrapperItem {
    let task_definition = GenTaskDefinition::subtask(
        WorkerPlugin::Basic,
        &env::full_path_curr_dir("src/tasks/js/subtask.js"),
        params,
        parent_task_uuid,
        name
    );
    Box::new(Subtask::new(task_definition))
}

