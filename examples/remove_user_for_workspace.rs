#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let workspace_gid = "your workspace gid";
    let response = client
        .remove_user_for_workspace(workspace_gid)
        .data(WorkspaceRemoveUserRequest {
            user: Some("your user".to_owned()),
        })
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}