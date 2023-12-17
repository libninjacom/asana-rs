#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let goal_gid = "your goal gid";
    let response = client
        .update_goal_metric(goal_gid)
        .data(GoalMetricCurrentValueRequest {
            asana_resource: AsanaResource {
                gid: Some("your gid".to_owned()),
                resource_type: Some("your resource type".to_owned()),
            },
            current_number_value: Some(1.0),
        })
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}