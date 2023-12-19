#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = CreateTimeTrackingEntryRequestBody {
        duration_minutes: 1,
        entered_on: chrono::Utc::now().date_naive(),
    };
    let task_gid = "your task gid";
    let response = client
        .create_time_tracking_entry(data, task_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}