#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let project_gid = "your project gid";
    let response = client
        .get_project_memberships_for_project(project_gid)
        .limit(1)
        .offset("your offset")
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .user("your user")
        .await
        .unwrap();
    println!("{:#?}", response);
}