#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let workspace_gid = "your workspace gid";
    let response = client
        .update_workspace(workspace_gid)
        .data(WorkspaceCompact {
            asana_resource: AsanaResource {
                gid: Some("your gid".to_owned()),
                resource_type: Some("your resource type".to_owned()),
            },
            name: Some("your name".to_owned()),
        })
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}