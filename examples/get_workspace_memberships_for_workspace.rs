#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let workspace_gid = "your workspace gid";
    let response = client
        .get_workspace_memberships_for_workspace(workspace_gid)
        .limit(1)
        .offset("your offset")
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .user("your user")
        .await
        .unwrap();
    println!("{:#?}", response);
}
