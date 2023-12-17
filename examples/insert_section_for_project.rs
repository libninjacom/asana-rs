#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let project_gid = "your project gid";
    let response = client
        .insert_section_for_project(project_gid)
        .data(ProjectSectionInsertRequest {
            after_section: Some("your after section".to_owned()),
            before_section: Some("your before section".to_owned()),
            section: "your section".to_owned(),
        })
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}