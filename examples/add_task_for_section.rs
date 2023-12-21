#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = SectionTaskInsertRequest {
        insert_after: Some("your insert after".to_owned()),
        insert_before: Some("your insert before".to_owned()),
        task: "your task".to_owned(),
    };
    let section_gid = "your section gid";
    let response = client
        .add_task_for_section(data, section_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}