#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = StatusUpdateRequest {
        status_update_base: StatusUpdateBase {
            status_update_compact: StatusUpdateCompact {
                asana_resource: AsanaResource {
                    gid: Some("your gid".to_owned()),
                    resource_type: Some("your resource type".to_owned()),
                },
                resource_subtype: Some("your resource subtype".to_owned()),
                title: Some("your title".to_owned()),
            },
            html_text: Some("your html text".to_owned()),
            status_type: "your status type".to_owned(),
            text: "your text".to_owned(),
        },
        parent: "your parent".to_owned(),
    };
    let response = client
        .create_status_for_object(data)
        .limit(1)
        .offset("your offset")
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}