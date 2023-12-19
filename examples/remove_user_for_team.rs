#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = TeamRemoveUserRequest {
        user: Some("your user".to_owned()),
    };
    let team_gid = "your team gid";
    let response = client
        .remove_user_for_team(data, team_gid)
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}