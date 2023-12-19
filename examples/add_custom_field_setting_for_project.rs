#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = AddCustomFieldSettingRequest {
        custom_field: "your custom field".to_owned(),
        insert_after: "your insert after".to_owned(),
        insert_before: "your insert before".to_owned(),
        is_important: true,
    };
    let project_gid = "your project gid";
    let response = client
        .add_custom_field_setting_for_project(data, project_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
