#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let tag_gid = "your tag gid";
    let response = client.delete_tag(tag_gid).opt_pretty(true).await.unwrap();
    println!("{:#?}", response);
}