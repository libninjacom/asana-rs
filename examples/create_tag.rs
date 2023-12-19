#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = TagRequest {
        tag_base: TagBase {
            tag_compact: TagCompact {
                asana_resource: AsanaResource {
                    gid: "your gid".to_owned(),
                    resource_type: "your resource type".to_owned(),
                },
                name: Some("your name".to_owned()),
            },
            color: Some(serde_json::json!({})),
            notes: Some("your notes".to_owned()),
        },
        followers: Some(vec!["your followers".to_owned()]),
        workspace: Some("your workspace".to_owned()),
    };
    let response = client
        .create_tag(data)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
