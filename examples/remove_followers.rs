#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = TaskAddFollowersRequest {
        followers: vec!["your followers".to_owned()],
    };
    let goal_gid = "your goal gid";
    let response = client
        .remove_followers(data, goal_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
