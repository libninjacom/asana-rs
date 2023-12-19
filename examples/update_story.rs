#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = StoryBase {
        asana_resource: AsanaResource {
            gid: Some("your gid".to_owned()),
            resource_type: Some("your resource type".to_owned()),
        },
        created_at: Some(chrono::Utc::now()),
        html_text: Some("your html text".to_owned()),
        is_pinned: Some(true),
        resource_subtype: Some("your resource subtype".to_owned()),
        sticker_name: Some("your sticker name".to_owned()),
        text: Some("your text".to_owned()),
    };
    let story_gid = "your story gid";
    let response = client
        .update_story(data, story_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}