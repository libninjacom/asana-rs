#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = ProjectRequest {
        project_base: ProjectBase {
            project_compact: ProjectCompact {
                asana_resource: AsanaResource {
                    gid: Some("your gid".to_owned()),
                    resource_type: Some("your resource type".to_owned()),
                },
                name: Some("your name".to_owned()),
            },
            archived: Some(true),
            color: Some(serde_json::json!({})),
            created_at: Some(chrono::Utc::now()),
            current_status: Some(serde_json::json!({})),
            current_status_update: Some(serde_json::json!({})),
            custom_field_settings: Some(
                vec![
                    CustomFieldSettingResponse { custom_field_setting_base :
                    AsanaResource { gid : Some("your gid".to_owned()), resource_type :
                    Some("your resource type".to_owned()) }, custom_field :
                    Some(serde_json::json!({})), is_important : Some(true), parent :
                    Some(serde_json::json!({})), project : Some(serde_json::json!({})) }
                ],
            ),
            default_access_level: Some("your default access level".to_owned()),
            default_view: Some("your default view".to_owned()),
            due_date: Some(chrono::Utc::now().date_naive()),
            due_on: Some(chrono::Utc::now().date_naive()),
            html_notes: Some("your html notes".to_owned()),
            members: Some(
                vec![
                    UserCompact { asana_resource : AsanaResource { gid : Some("your gid"
                    .to_owned()), resource_type : Some("your resource type".to_owned())
                    }, name : Some("your name".to_owned()) }
                ],
            ),
            minimum_access_level_for_customization: Some(
                "your minimum access level for customization".to_owned(),
            ),
            minimum_access_level_for_sharing: Some(
                "your minimum access level for sharing".to_owned(),
            ),
            modified_at: Some(chrono::Utc::now()),
            notes: Some("your notes".to_owned()),
            public: Some(true),
            start_on: Some(chrono::Utc::now().date_naive()),
        },
        custom_fields: Some(serde_json::json!({})),
        followers: Some("your followers".to_owned()),
        owner: Some("your owner".to_owned()),
        team: Some("your team".to_owned()),
        workspace: Some("your workspace".to_owned()),
    };
    let workspace_gid = "your workspace gid";
    let response = client
        .create_project_for_workspace(data, workspace_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}