#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let webhook_gid = "your webhook gid";
    let response = client
        .get_webhook(webhook_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}