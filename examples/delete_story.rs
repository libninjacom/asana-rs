#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let story_gid = "your story gid";
    let response = client.delete_story(story_gid).opt_pretty(true).await.unwrap();
    println!("{:#?}", response);
}