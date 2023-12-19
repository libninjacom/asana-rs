#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let project_template_gid = "your project template gid";
    let response = client
        .delete_project_template(project_template_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
