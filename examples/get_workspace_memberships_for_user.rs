#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let user_gid = "your user gid";
    let response = client
        .get_workspace_memberships_for_user(user_gid)
        .limit(1)
        .offset("your offset")
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}