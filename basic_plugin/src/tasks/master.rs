use actix::prelude::*;
use serde_json;
use slog::Logger;
use std::collections::HashSet;

use patoka::{
    center::send::*,
    control::message::StopTask,
    core::logger::create_logger,
    core::env,
    worker::{
        client::{WorkerClient, ClientContext, GenClientContext},
        controller,
        plugin::{WorkerPlugin},
        processor::{self, *},
        setup::*,
        task::{WorkerTask, GenTaskDefinition, TaskStatus},
        tracker::{self, *},
        worker_message::{WorkerMessage},
    },
};

use tasks::subtask;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterTaskParams {
    /// Generate `number` numbers from the range 0..`max`.

    pub number: usize,
    pub max: usize,
}

impl MasterTaskParams {
    pub fn new(number: usize, max: usize) -> Self {
        Self {
            number,
            max,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MasterTaskExecutionResult {
    pub generated_numbers: Vec<usize>,
}

#[derive(Clone)]
pub struct MasterClient {
    log: Logger,
    ctx: GenClientContext<MasterTaskParams>,

    /// Subtasks' UUIDs.
    created_subtasks: HashSet<String>,
    finished_subtasks: HashSet<String>,
}

impl Actor for MasterClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!(self.log, "Demo Master Client started.");

        setup_with_controller(
            &self.ctx.task_definition.task_uuid, /* Task UUID */
            None, /* ControlMessage */
            Some(ctx.address().recipient()), /* TaskUpdate */
            ctx.address().recipient(), /* WorkerMessage */
            &self.ctx.controller_addr, /* Controller */
            self.ctx.task_definition.make_message(), /* Message to start */
            self.ctx.task_definition.name.clone(),
        );
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!(self.log, "Demo Master Client stopped.");

        send_center_task_finished(
            &self.ctx.task_uuid,
            TaskStatus::FinishedSuccess,
            &self.ctx.task_definition.name,
        );
    }
}

impl MasterClient {
    fn process_task_result(
        &mut self,
        res: MasterTaskExecutionResult,
        own_addr: Addr<MasterClient>,
    ) {
        debug!(self.log, "Generated numbers: {:?}", res.generated_numbers);

        let connector = processor::start();
        let task_tracker = tracker::start();

        // Create a subtask for every generated number.
        let mut i = -1;
        for number in res.generated_numbers {
            let params = subtask::SubtaskParams { number };

            i += 1;
            let name = format!("subtask_{}", i);

            let subtask = subtask::create(
                params,
                self.ctx.task_uuid.clone(),
                &name,
            );

            self.created_subtasks.insert(subtask.uuid().to_string());

            connector.do_send(TaskWrapperItemMessage(subtask));
        }
    }

    fn unsubscribe_from_subtasks(
        &self,
        own_addr: Addr<MasterClient>,
    ) {
        let task_tracker = tracker::start();
        for subtask_uuid in &self.created_subtasks {
            let task_subscription = TaskSubscription::unsubscribe(
                subtask_uuid.to_string(),
                self.ctx.task_uuid.clone(),
            );
            task_tracker.do_send(task_subscription);
        }
    }

    fn handle_worker_message(
        &mut self,
        msg: WorkerMessage,
        ctx: &mut <Self as Actor>::Context
    ) {
        debug!(self.log, "Received a worker message.");

        if let Some(task_result) = msg.payload.data.get("task_result") {
            self.process_task_result(
                serde_json::from_value(task_result.clone()).unwrap(),
                ctx.address(),
            );
        }
    }
}

impl WorkerClient for MasterClient {
    type TaskDefinition = GenTaskDefinition<MasterTaskParams>;

    fn new(ctx: ClientContext<Self::TaskDefinition>) -> Self {
        MasterClient {
            log: create_logger("master"),
            ctx,
            created_subtasks: HashSet::new(),
            finished_subtasks: HashSet::new(),
        }
    }
}

impl Handler<TaskUpdate> for MasterClient {
    type Result = ();

    fn handle(
        &mut self,
        msg: TaskUpdate,
        ctx: &mut Self::Context
    ) -> Self::Result {
        debug!(self.log, "Received status update for task {}", msg.task_uuid);

        match msg.status {
            TaskStatus::FinishedSuccess | TaskStatus::FinishedFailure => {
                if self.created_subtasks.contains(&msg.task_uuid) {
                    self.finished_subtasks.insert(msg.task_uuid);
                    debug!(
                        self.log,
                        "{} of {} subtasks completed.",
                        self.finished_subtasks.len(),
                        self.created_subtasks.len(),
                    );
                }
            },
            _ => { }
        }

        if self.finished_subtasks.len() == self.created_subtasks.len() {
            // All subtasks are completed.
            self.unsubscribe_from_subtasks(ctx.address());

            // Done.
            ctx.stop();
        }
    }
}

handler_impl_worker_message!(MasterClient);

handler_impl_stop_task!(MasterClient);

type MasterTask = WorkerTask<MasterClient>;

pub fn start() {
    let params = env::load_params::<MasterTaskParams>("basic_plugin");
    let connector = processor::start();
    let task_definition = GenTaskDefinition::new(
        WorkerPlugin::Basic,
        &env::full_path_curr_dir("src/tasks/js/master.js"),
        params,
        "master"
    );
    let master_task = MasterTask::new(task_definition);
    connector.do_send(TaskWrapperItemMessage(Box::new(master_task)));
}
