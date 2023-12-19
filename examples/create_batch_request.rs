#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = BatchRequest {
        actions: Some(
            vec![
                BatchRequestAction { data : Some(serde_json::json!({})), method :
                "your method".to_owned(), options : Some(serde_json::json!({})),
                relative_path : "your relative path".to_owned() }
            ],
        ),
    };
    let response = client
        .create_batch_request(data)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}