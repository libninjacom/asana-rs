#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let project_gid = "your project gid";
    let response = client
        .duplicate_project(project_gid)
        .data(ProjectDuplicateRequest {
            include: Some("your include".to_owned()),
            name: "your name".to_owned(),
            schedule_dates: Some(serde_json::json!({})),
            team: Some("your team".to_owned()),
        })
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}