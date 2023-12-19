#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let supported_goal = "your supported goal";
    let response = client
        .get_goal_relationships(supported_goal)
        .limit(1)
        .offset("your offset")
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .resource_subtype("your resource subtype")
        .await
        .unwrap();
    println!("{:#?}", response);
}
