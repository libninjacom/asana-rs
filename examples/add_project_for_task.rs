#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = TaskAddProjectRequest {
        insert_after: Some("your insert after".to_owned()),
        insert_before: Some("your insert before".to_owned()),
        project: "your project".to_owned(),
        section: Some("your section".to_owned()),
    };
    let task_gid = "your task gid";
    let response = client
        .add_project_for_task(data, task_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
