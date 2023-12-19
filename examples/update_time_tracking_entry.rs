#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = UpdateTimeTrackingEntryRequestBody {
        duration_minutes: 1,
        entered_on: chrono::Utc::now().date_naive(),
    };
    let time_tracking_entry_gid = "your time tracking entry gid";
    let response = client
        .update_time_tracking_entry(data, time_tracking_entry_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
