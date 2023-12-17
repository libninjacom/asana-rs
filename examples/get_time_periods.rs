#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let workspace = "your workspace";
    let response = client
        .get_time_periods(workspace)
        .end_on(chrono::Utc::now().date_naive())
        .limit(1)
        .offset("your offset")
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .start_on(chrono::Utc::now().date_naive())
        .await
        .unwrap();
    println!("{:#?}", response);
}