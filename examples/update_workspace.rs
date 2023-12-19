#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = WorkspaceCompact {
        asana_resource: AsanaResource {
            gid: "your gid".to_owned(),
            resource_type: "your resource type".to_owned(),
        },
        name: Some("your name".to_owned()),
    };
    let workspace_gid = "your workspace gid";
    let response = client
        .update_workspace(data, workspace_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}