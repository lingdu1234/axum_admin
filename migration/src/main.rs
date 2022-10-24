use sea_orm_migration::prelude::*;

use dotenv;
use std::env::{self, Vars};
#[async_std::main]
async fn main() {
    let _xx = dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("Environment variable 'DATABASE_URL' not set");
    println!("DATABASE_URL:{:?}", url);
    cli::run_cli(migration::Migrator).await;
}
