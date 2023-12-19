#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let project_status_gid = "your project status gid";
    let response = client
        .delete_project_status(project_status_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
