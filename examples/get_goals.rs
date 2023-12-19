#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let response = client
        .get_goals()
        .is_workspace_level(true)
        .limit(1)
        .offset("your offset")
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .portfolio("your portfolio")
        .project("your project")
        .task("your task")
        .team("your team")
        .time_periods(&["your time periods"])
        .workspace("your workspace")
        .await
        .unwrap();
    println!("{:#?}", response);
}
