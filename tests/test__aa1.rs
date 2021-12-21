

use chrono::{Local, NaiveDateTime};


#[tokio::test]
async fn test_time() {
    let a = Local::now().to_string();
    println!("{}", a);

    let now: NaiveDateTime = Local::now().naive_local();
    println!("{:?}", now);
}
