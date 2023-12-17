#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let enum_option_gid = "your enum option gid";
    let response = client
        .update_enum_option(enum_option_gid)
        .data(EnumOption {
            asana_resource: AsanaResource {
                gid: Some("your gid".to_owned()),
                resource_type: Some("your resource type".to_owned()),
            },
            color: Some("your color".to_owned()),
            enabled: Some(true),
            name: Some("your name".to_owned()),
        })
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}