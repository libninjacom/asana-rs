#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let portfolio_gid = "your portfolio gid";
    let response = client
        .delete_portfolio(portfolio_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
