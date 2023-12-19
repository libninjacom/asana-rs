#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = GoalAddSupportingRelationshipRequest {
        contribution_weight: Some(1.0),
        insert_after: Some("your insert after".to_owned()),
        insert_before: Some("your insert before".to_owned()),
        supporting_resource: "your supporting resource".to_owned(),
    };
    let goal_gid = "your goal gid";
    let response = client
        .add_supporting_relationship(data, goal_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}