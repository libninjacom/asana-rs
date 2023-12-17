#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let goal_gid = "your goal gid";
    let response = client
        .remove_supporting_relationship(goal_gid)
        .data(GoalRemoveSupportingRelationshipRequest {
            supporting_resource: "your supporting resource".to_owned(),
        })
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}