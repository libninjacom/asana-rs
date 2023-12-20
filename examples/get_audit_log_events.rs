#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let workspace_gid = "your workspace gid";
    let response = client
        .get_audit_log_events(workspace_gid)
        .actor_gid("your actor gid")
        .actor_type("your actor type")
        .end_at(chrono::Utc::now())
        .event_type("your event type")
        .limit(1)
        .offset("your offset")
        .resource_gid("your resource gid")
        .start_at(chrono::Utc::now())
        .await
        .unwrap();
    println!("{:#?}", response);
}