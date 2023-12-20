#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let response = client
        .get_portfolio_memberships()
        .limit(1)
        .offset("your offset")
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .portfolio("your portfolio")
        .user("your user")
        .workspace("your workspace")
        .await
        .unwrap();
    println!("{:#?}", response);
}