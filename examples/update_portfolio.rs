#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = PortfolioRequest {
        portfolio_base: PortfolioBase {
            portfolio_compact: PortfolioCompact {
                asana_resource: AsanaResource {
                    gid: Some("your gid".to_owned()),
                    resource_type: Some("your resource type".to_owned()),
                },
                name: Some("your name".to_owned()),
            },
            color: Some("your color".to_owned()),
        },
        members: Some(vec!["your members".to_owned()]),
        public: Some(true),
        workspace: Some("your workspace".to_owned()),
    };
    let portfolio_gid = "your portfolio gid";
    let response = client
        .update_portfolio(data, portfolio_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}