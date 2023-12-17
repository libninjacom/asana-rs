#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let portfolio_gid = "your portfolio gid";
    let response = client
        .add_item_for_portfolio(portfolio_gid)
        .data(PortfolioAddItemRequest {
            insert_after: Some("your insert after".to_owned()),
            insert_before: Some("your insert before".to_owned()),
            item: "your item".to_owned(),
        })
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}