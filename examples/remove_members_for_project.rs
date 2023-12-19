#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = RemoveMembersRequest {
        members: "your members".to_owned(),
    };
    let project_gid = "your project gid";
    let response = client
        .remove_members_for_project(data, project_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
