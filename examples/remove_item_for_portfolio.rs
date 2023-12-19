#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = PortfolioRemoveItemRequest {
        item: "your item".to_owned(),
    };
    let portfolio_gid = "your portfolio gid";
    let response = client
        .remove_item_for_portfolio(data, portfolio_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
