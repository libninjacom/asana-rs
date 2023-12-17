#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let goal_gid = "your goal gid";
    let response = client.delete_goal(goal_gid).opt_pretty(true).await.unwrap();
    println!("{:#?}", response);
}