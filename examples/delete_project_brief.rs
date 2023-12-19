#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let project_brief_gid = "your project brief gid";
    let response = client
        .delete_project_brief(project_brief_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
