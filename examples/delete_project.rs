#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let project_gid = "your project gid";
    let response = client.delete_project(project_gid).opt_pretty(true).await.unwrap();
    println!("{:#?}", response);
}