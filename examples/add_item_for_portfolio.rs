#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = PortfolioAddItemRequest {
        insert_after: "your insert after".to_owned(),
        insert_before: "your insert before".to_owned(),
        item: "your item".to_owned(),
    };
    let portfolio_gid = "your portfolio gid";
    let response = client
        .add_item_for_portfolio(data, portfolio_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}