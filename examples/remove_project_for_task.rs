#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = TaskRemoveProjectRequest {
        project: "your project".to_owned(),
    };
    let task_gid = "your task gid";
    let response = client
        .remove_project_for_task(data, task_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
