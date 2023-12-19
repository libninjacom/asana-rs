#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = TaskRequest {
        task_base: TaskBase {
            task_compact: TaskCompact {
                asana_resource: AsanaResource {
                    gid: Some("your gid".to_owned()),
                    resource_type: Some("your resource type".to_owned()),
                },
                created_by: Some(serde_json::json!({})),
                name: Some("your name".to_owned()),
                resource_subtype: Some("your resource subtype".to_owned()),
            },
            actual_time_minutes: Some(1.0),
            approval_status: Some("your approval status".to_owned()),
            assignee_status: Some("your assignee status".to_owned()),
            completed: Some(true),
            completed_at: Some(chrono::Utc::now()),
            completed_by: Some(serde_json::json!({})),
            created_at: Some(chrono::Utc::now()),
            dependencies: Some(
                vec![
                    AsanaResource { gid : Some("your gid".to_owned()), resource_type :
                    Some("your resource type".to_owned()) }
                ],
            ),
            dependents: Some(
                vec![
                    AsanaResource { gid : Some("your gid".to_owned()), resource_type :
                    Some("your resource type".to_owned()) }
                ],
            ),
            due_at: Some(chrono::Utc::now()),
            due_on: Some(chrono::Utc::now().date_naive()),
            external: Some(serde_json::json!({})),
            hearted: Some(true),
            hearts: Some(
                vec![
                    Like { gid : Some("your gid".to_owned()), user : Some(UserCompact {
                    asana_resource : AsanaResource { gid : Some("your gid".to_owned()),
                    resource_type : Some("your resource type".to_owned()) }, name :
                    Some("your name".to_owned()) }) }
                ],
            ),
            html_notes: Some("your html notes".to_owned()),
            is_rendered_as_separator: Some(true),
            liked: Some(true),
            likes: Some(
                vec![
                    Like { gid : Some("your gid".to_owned()), user : Some(UserCompact {
                    asana_resource : AsanaResource { gid : Some("your gid".to_owned()),
                    resource_type : Some("your resource type".to_owned()) }, name :
                    Some("your name".to_owned()) }) }
                ],
            ),
            memberships: Some(vec![serde_json::json!({})]),
            modified_at: Some(chrono::Utc::now()),
            name: Some("your name".to_owned()),
            notes: Some("your notes".to_owned()),
            num_hearts: Some(1),
            num_likes: Some(1),
            num_subtasks: Some(1),
            start_at: Some(chrono::Utc::now()),
            start_on: Some(chrono::Utc::now().date_naive()),
        },
        assignee: Some("your assignee".to_owned()),
        assignee_section: Some("your assignee section".to_owned()),
        custom_fields: Some(serde_json::json!({})),
        followers: Some(vec!["your followers".to_owned()]),
        parent: Some("your parent".to_owned()),
        projects: Some(vec!["your projects".to_owned()]),
        tags: Some(vec!["your tags".to_owned()]),
        workspace: Some("your workspace".to_owned()),
    };
    let task_gid = "your task gid";
    let response = client
        .update_task(data, task_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}