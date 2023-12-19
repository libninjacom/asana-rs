#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let attachment_gid = "your attachment gid";
    let response = client
        .delete_attachment(attachment_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
