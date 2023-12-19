#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = TagCreateTagForWorkspaceRequest {
        tag_base: TagBase {
            tag_compact: TagCompact {
                asana_resource: AsanaResource {
                    gid: Some("your gid".to_owned()),
                    resource_type: Some("your resource type".to_owned()),
                },
                name: Some("your name".to_owned()),
            },
            color: Some(serde_json::json!({})),
            notes: Some("your notes".to_owned()),
        },
        followers: Some(vec!["your followers".to_owned()]),
    };
    let workspace_gid = "your workspace gid";
    let response = client
        .create_tag_for_workspace(data, workspace_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}