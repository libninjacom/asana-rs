#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let rule_trigger_gid = "your rule trigger gid";
    let response = client
        .trigger_rule(rule_trigger_gid)
        .data(RuleTriggerRequest {
            action_data: serde_json::json!({}),
            resource: "your resource".to_owned(),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}