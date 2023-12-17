#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let task_gid = "your task gid";
    let response = client
        .remove_project_for_task(task_gid)
        .data(TaskRemoveProjectRequest {
            project: "your project".to_owned(),
        })
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}