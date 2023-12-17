#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let team_gid = "your team gid";
    let response = client
        .update_team(team_gid)
        .data(TeamRequest {
            team_base: TeamCompact {
                asana_resource: AsanaResource {
                    gid: Some("your gid".to_owned()),
                    resource_type: Some("your resource type".to_owned()),
                },
                name: Some("your name".to_owned()),
            },
            description: Some("your description".to_owned()),
            edit_team_name_or_description_access_level: Some(
                "your edit team name or description access level".to_owned(),
            ),
            edit_team_visibility_or_trash_team_access_level: Some(
                "your edit team visibility or trash team access level".to_owned(),
            ),
            guest_invite_management_access_level: Some(
                "your guest invite management access level".to_owned(),
            ),
            html_description: Some("your html description".to_owned()),
            join_request_management_access_level: Some(
                "your join request management access level".to_owned(),
            ),
            member_invite_management_access_level: Some(
                "your member invite management access level".to_owned(),
            ),
            organization: Some("your organization".to_owned()),
            team_member_removal_access_level: Some(
                "your team member removal access level".to_owned(),
            ),
            visibility: Some("your visibility".to_owned()),
        })
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}