#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let team_gid = "your team gid";
    let response = client
        .remove_user_for_team(team_gid)
        .data(TeamRemoveUserRequest {
            user: Some("your user".to_owned()),
        })
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}