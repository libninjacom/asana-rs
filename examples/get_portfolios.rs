#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let workspace = "your workspace";
    let response = client
        .get_portfolios(workspace)
        .limit(1)
        .offset("your offset")
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .owner("your owner")
        .await
        .unwrap();
    println!("{:#?}", response);
}
