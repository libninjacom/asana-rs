#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let task_gid = "your task gid";
    let response = client
        .add_dependents_for_task(task_gid)
        .data(ModifyDependentsRequest {
            dependents: Some(vec!["your dependents".to_owned()]),
        })
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}