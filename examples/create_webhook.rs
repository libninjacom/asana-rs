#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let response = client
        .create_webhook()
        .data(WebhookRequest {
            filters: Some(vec![serde_json::json!({})]),
            resource: "your resource".to_owned(),
            target: "your target".to_owned(),
        })
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}