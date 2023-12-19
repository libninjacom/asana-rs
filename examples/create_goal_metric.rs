#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = GoalMetricBase {
        asana_resource: AsanaResource {
            gid: Some("your gid".to_owned()),
            resource_type: Some("your resource type".to_owned()),
        },
        currency_code: Some("your currency code".to_owned()),
        current_display_value: Some("your current display value".to_owned()),
        current_number_value: Some(1.0),
        initial_number_value: Some(1.0),
        precision: Some(1),
        progress_source: Some("your progress source".to_owned()),
        resource_subtype: Some("your resource subtype".to_owned()),
        target_number_value: Some(1.0),
        unit: Some("your unit".to_owned()),
    };
    let goal_gid = "your goal gid";
    let response = client
        .create_goal_metric(data, goal_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}