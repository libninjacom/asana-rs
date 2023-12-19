#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = TeamAddUserRequest {
        user: "your user".to_owned(),
    };
    let team_gid = "your team gid";
    let response = client
        .add_user_for_team(data, team_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
