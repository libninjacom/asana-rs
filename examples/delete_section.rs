#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let section_gid = "your section gid";
    let response = client.delete_section(section_gid).opt_pretty(true).await.unwrap();
    println!("{:#?}", response);
}
