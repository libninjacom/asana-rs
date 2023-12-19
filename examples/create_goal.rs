#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = GoalRequest {
        goal_request_base: GoalRequestBase {
            goal_base: GoalBase {
                asana_resource: AsanaResource {
                    gid: "your gid".to_owned(),
                    resource_type: "your resource type".to_owned(),
                },
                due_on: Some("your due on".to_owned()),
                html_notes: Some("your html notes".to_owned()),
                is_workspace_level: Some(true),
                liked: Some(true),
                name: Some("your name".to_owned()),
                notes: Some("your notes".to_owned()),
                start_on: Some("your start on".to_owned()),
            },
            owner: Some("your owner".to_owned()),
            team: Some("your team".to_owned()),
            time_period: Some("your time period".to_owned()),
            workspace: Some("your workspace".to_owned()),
        },
        followers: Some(vec!["your followers".to_owned()]),
    };
    let response = client
        .create_goal(data)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}