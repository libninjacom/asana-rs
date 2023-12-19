#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = ProjectSaveAsTemplateRequestBody {
        name: "your name".to_owned(),
        public: true,
        team: Some("your team".to_owned()),
        workspace: Some("your workspace".to_owned()),
    };
    let project_gid = "your project gid";
    let response = client
        .project_save_as_template(data, project_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}