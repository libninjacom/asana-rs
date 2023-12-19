#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = WorkspaceRemoveUserRequest {
        user: Some("your user".to_owned()),
    };
    let workspace_gid = "your workspace gid";
    let response = client
        .remove_user_for_workspace(data, workspace_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}