#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let time_tracking_entry_gid = "your time tracking entry gid";
    let response = client
        .delete_time_tracking_entry(time_tracking_entry_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
