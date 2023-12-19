#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = TaskTemplateInstantiateTaskRequest {
        name: Some("your name".to_owned()),
    };
    let task_template_gid = "your task template gid";
    let response = client
        .instantiate_task(data, task_template_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}