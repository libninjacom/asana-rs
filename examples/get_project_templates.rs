#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let response = client
        .get_project_templates()
        .limit(1)
        .offset("your offset")
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .team("your team")
        .workspace("your workspace")
        .await
        .unwrap();
    println!("{:#?}", response);
}