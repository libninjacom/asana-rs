#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let team_gid = "your team gid";
    let response = client
        .get_team(team_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
