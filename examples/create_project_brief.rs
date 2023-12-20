#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = ProjectBriefRequest {
        project_brief_base: ProjectBriefBase {
            project_brief_compact: AsanaResource {
                gid: "your gid".to_owned(),
                resource_type: "your resource type".to_owned(),
            },
            html_text: Some("your html text".to_owned()),
            title: Some("your title".to_owned()),
        },
        text: Some("your text".to_owned()),
    };
    let project_gid = "your project gid";
    let response = client
        .create_project_brief(data, project_gid)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}