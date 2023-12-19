#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = RuleTriggerRequest {
        action_data: serde_json::json!({}),
        resource: "your resource".to_owned(),
    };
    let rule_trigger_gid = "your rule trigger gid";
    let response = client.trigger_rule(data, rule_trigger_gid).await.unwrap();
    println!("{:#?}", response);
}
