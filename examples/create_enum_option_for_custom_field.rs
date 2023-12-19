#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let custom_field_gid = "your custom field gid";
    let data = EnumOptionRequest {
        enum_option_base: EnumOption {
            asana_resource: AsanaResource {
                gid: Some("your gid".to_owned()),
                resource_type: Some("your resource type".to_owned()),
            },
            color: Some("your color".to_owned()),
            enabled: Some(true),
            name: Some("your name".to_owned()),
        },
        insert_after: Some("your insert after".to_owned()),
        insert_before: Some("your insert before".to_owned()),
    };
    let response = client
        .create_enum_option_for_custom_field(custom_field_gid, data)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}