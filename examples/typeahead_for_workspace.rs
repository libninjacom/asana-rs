#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let resource_type = "your resource type";
    let workspace_gid = "your workspace gid";
    let response = client
        .typeahead_for_workspace(resource_type, workspace_gid)
        .count(1)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .query("your query")
        .type_("your type")
        .await
        .unwrap();
    println!("{:#?}", response);
}