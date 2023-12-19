#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = ProjectTemplateInstantiateProjectRequest {
        is_strict: true,
        name: "your name".to_owned(),
        public: true,
        requested_dates: vec![
            DateVariableRequest { gid : "your gid".to_owned(), value :
            Some(chrono::Utc::now()) }
        ],
        requested_roles: vec![
            RequestedRoleRequest { gid : "your gid".to_owned(), value : "your value"
            .to_owned() }
        ],
        team: "your team".to_owned(),
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