use chrono::{Local, NaiveDateTime};
use scru128::scru128_string;

#[tokio::test]
async fn test_time() {
    let a = Local::now().to_string();
    println!("{}", a);

    let now: NaiveDateTime = Local::now().naive_local();
    println!("{:?}", now);
}

#[tokio::test]
async fn scu_test() {
    for i in 1..20 {
        println!("{}", scru128_string());
    }
}
