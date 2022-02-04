use delay_timer::anyhow::Result;
use delay_timer::prelude::*;
use delay_timer::{
    entity::DelayTimer,
    error::TaskError,
    prelude::{
        create_delay_task_handler, get_timestamp, DelayTaskHandler, DelayTimerBuilder, Task,
        TaskBuilder, TaskContext,
    },
    utils::convenience::async_template,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use once_cell::sync::Lazy;

pub static TASK_TIMER: Lazy<Arc<Mutex<DelayTimer>>> = Lazy::new(|| {
    let t_timeer = DelayTimerBuilder::default()
        .tokio_runtime_by_default()
        .build();
    Arc::new(Mutex::new(t_timeer))
});

pub fn get_task(
    cron_str: &str,
    task_name: &str,
    task_count: u64,
    task_id: u64,
) -> Result<Task, TaskError> {
    build_task_async_task(cron_str, task_name, task_count, task_id)
}

fn build_task_async_task(
    cron_str: &str,
    task_name: &str,
    task_count: u64,
    task_id: u64,
) -> Result<Task, TaskError> {
    let mut task_builder = TaskBuilder::default();

    let body = generate_task_by_closure(task_name.to_string());
    let task = match task_count {
        0 => task_builder
            .set_frequency_repeated_by_cron_str(cron_str)
            .set_task_id(task_id)
            .spawn(body)?,
        x => task_builder
            .set_frequency_count_down_by_cron_str(cron_str, x)
            .set_task_id(task_id)
            .spawn(body)?,
    };
    Ok(task)
}

pub fn generate_task_by_closure(
    task_name: String,
) -> impl Fn(TaskContext) -> Box<dyn DelayTaskHandler> + 'static + Send + Sync {
    move |context:TaskContext| {
        let t1 = task_name.clone();
        let t2 = task_name.clone();
        let future_inner = async_template(get_timestamp() as i32, t1);

        let future = async move {
            run_task(t2);
            future_inner.await.ok();
            context.finish_task(None).await;
        };

        create_delay_task_handler(async_spawn_by_tokio(future))
    }
}



fn run_task(task_name: String) {
    match task_name.as_str() {
        "a" => print_a(),
        "b" => print_b(),
        _ => println!("task name is not found"),
    }
}

fn print_a() {
    println!("a");
}

fn print_b() {
    println!("b");
}
