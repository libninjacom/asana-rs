#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = AddCustomFieldSettingRequest {
        custom_field: "your custom field".to_owned(),
        insert_after: Some("your insert after".to_owned()),
        insert_before: Some("your insert before".to_owned()),
        is_important: Some(true),
    };
    let portfolio_gid = "your portfolio gid";
    let response = client
        .add_custom_field_setting_for_portfolio(data, portfolio_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}