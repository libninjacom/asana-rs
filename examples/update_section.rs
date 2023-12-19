#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = SectionRequest {
        insert_after: "your insert after".to_owned(),
        insert_before: "your insert before".to_owned(),
        name: "your name".to_owned(),
    };
    let section_gid = "your section gid";
    let response = client
        .update_section(data, section_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
