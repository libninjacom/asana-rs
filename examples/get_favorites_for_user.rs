#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let resource_type = "your resource type";
    let user_gid = "your user gid";
    let workspace = "your workspace";
    let response = client
        .get_favorites_for_user(resource_type, user_gid, workspace)
        .limit(1)
        .offset("your offset")
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}