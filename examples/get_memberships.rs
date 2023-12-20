#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let response = client
        .get_memberships()
        .limit(1)
        .member("your member")
        .offset("your offset")
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .parent("your parent")
        .await
        .unwrap();
    println!("{:#?}", response);
}