#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let status_update_gid = "your status update gid";
    let response = client
        .delete_status(status_update_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
