#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let task_gid = "your task gid";
    let response = client
        .remove_tag_for_task(task_gid)
        .data(TaskRemoveTagRequest {
            tag: "your tag".to_owned(),
        })
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}