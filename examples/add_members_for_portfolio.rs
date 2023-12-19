#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = AddMembersRequest {
        members: "your members".to_owned(),
    };
    let portfolio_gid = "your portfolio gid";
    let response = client
        .add_members_for_portfolio(data, portfolio_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
