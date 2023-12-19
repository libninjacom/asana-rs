#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let custom_field_gid = "your custom field gid";
    let response = client
        .delete_custom_field(custom_field_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
