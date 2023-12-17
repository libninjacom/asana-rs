#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let membership_gid = "your membership gid";
    let response = client
        .delete_membership(membership_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}