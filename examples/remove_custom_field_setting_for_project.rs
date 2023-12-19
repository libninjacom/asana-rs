#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = RemoveCustomFieldSettingRequest {
        custom_field: "your custom field".to_owned(),
    };
    let project_gid = "your project gid";
    let response = client
        .remove_custom_field_setting_for_project(data, project_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
