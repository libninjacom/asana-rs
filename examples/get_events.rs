#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let resource = "your resource";
    let response = client
        .get_events(resource)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .sync("your sync")
        .await
        .unwrap();
    println!("{:#?}", response);
}