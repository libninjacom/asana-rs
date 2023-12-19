#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let custom_field_gid = "your custom field gid";
    let data = EnumOptionInsertRequest {
        after_enum_option: "your after enum option".to_owned(),
        before_enum_option: "your before enum option".to_owned(),
        enum_option: "your enum option".to_owned(),
    };
    let response = client
        .insert_enum_option_for_custom_field(custom_field_gid, data)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
