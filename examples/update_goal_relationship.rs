#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = GoalRelationshipBase {
        goal_relationship_compact: GoalRelationshipCompact {
            asana_resource: AsanaResource {
                gid: "your gid".to_owned(),
                resource_type: "your resource type".to_owned(),
            },
            contribution_weight: Some(1.0),
            resource_subtype: Some("your resource subtype".to_owned()),
            supporting_resource: Some(serde_json::json!({})),
        },
        supported_goal: Some(serde_json::json!({})),
    };
    let goal_relationship_gid = "your goal relationship gid";
    let response = client
        .update_goal_relationship(data, goal_relationship_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
