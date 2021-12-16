use std::time::Duration;

use chrono::{Local, NaiveDateTime};
use sea_orm::{sea_query::Query, ConnectOptions, Database};

#[tokio::test]
async fn test_time() {
    let a = Local::now().to_string();
    println!("{}", a);

    let now: NaiveDateTime = Local::now().naive_local();
    println!("{:?}", now);
}
