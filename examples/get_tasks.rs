#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let response = client
        .get_tasks()
        .assignee("your assignee")
        .completed_since(chrono::Utc::now())
        .limit(1)
        .modified_since(chrono::Utc::now())
        .offset("your offset")
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .project("your project")
        .section("your section")
        .workspace("your workspace")
        .await
        .unwrap();
    println!("{:#?}", response);
}
