#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = ProjectTemplateInstantiateProjectRequest {
        is_strict: Some(true),
        name: "your name".to_owned(),
        public: Some(true),
        requested_dates: Some(
            vec![
                DateVariableRequest { gid : Some("your gid".to_owned()), value :
                Some(chrono::Utc::now()) }
            ],
        ),
        requested_roles: Some(
            vec![
                RequestedRoleRequest { gid : Some("your gid".to_owned()), value :
                Some("your value".to_owned()) }
            ],
        ),
        team: Some("your team".to_owned()),
    };
    let project_template_gid = "your project template gid";
    let response = client
        .instantiate_project(data, project_template_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}