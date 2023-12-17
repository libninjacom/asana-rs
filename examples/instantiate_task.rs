#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let task_template_gid = "your task template gid";
    let response = client
        .instantiate_task(task_template_gid)
        .data(TaskTemplateInstantiateTaskRequest {
            name: Some("your name".to_owned()),
        })
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}