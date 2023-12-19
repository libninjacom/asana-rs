#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = TaskDuplicateRequest {
        include: "your include".to_owned(),
        name: "your name".to_owned(),
    };
    let task_gid = "your task gid";
    let response = client
        .duplicate_task(data, task_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
