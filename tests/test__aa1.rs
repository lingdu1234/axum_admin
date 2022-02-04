use chrono::{Duration, Local, NaiveDateTime, Utc};
use delay_timer::anyhow::Result;
use delay_timer::prelude::*;
use delay_timer::{
    error::TaskError,
    prelude::{
        create_delay_task_handler, get_timestamp, DelayTaskHandler, DelayTimerBuilder, Task,
        TaskBuilder, TaskContext,
    },
    utils::convenience::async_template,
};
use scru128::scru128_string;
use std::thread::{current, park, Thread};

#[tokio::test]
async fn test_time() {
    let a = Local::now().to_string();
    println!("{}", a);

    let now: NaiveDateTime = Local::now().naive_local();
    println!("{:?}", now);
}

#[tokio::test]
async fn scu_test() {
    for _i in 1..20 {
        println!("{}", scru128_string());
    }
}
#[tokio::test]
async fn timestamp_a() {
    let iat = Utc::now();
    let exp = iat + Duration::minutes(60);

    println!(
        "{:#?}----{:#?},----{:#?}",
        iat.timestamp(),
        exp.timestamp(),
        Duration::minutes(60)
    );
}
