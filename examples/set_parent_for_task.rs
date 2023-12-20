#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = TaskSetParentRequest {
        insert_after: "your insert after".to_owned(),
        insert_before: "your insert before".to_owned(),
        parent: "your parent".to_owned(),
    };
    let task_gid = "your task gid";
    let response = client
        .set_parent_for_task(data, task_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}