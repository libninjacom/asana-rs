#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let task_gid = "your task gid";
    let response = client
        .set_parent_for_task(task_gid)
        .data(TaskSetParentRequest {
            insert_after: Some("your insert after".to_owned()),
            insert_before: Some("your insert before".to_owned()),
            parent: "your parent".to_owned(),
        })
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}