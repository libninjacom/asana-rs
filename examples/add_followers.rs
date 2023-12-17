#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let goal_gid = "your goal gid";
    let response = client
        .add_followers(goal_gid)
        .data(TaskAddFollowersRequest {
            followers: vec!["your followers".to_owned()],
        })
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}