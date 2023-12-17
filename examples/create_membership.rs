#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let response = client
        .create_membership()
        .data(CreateMembershipRequestBody {
            membership_request: MembershipRequest {
                is_active: Some(true),
            },
            member: Some("your member".to_owned()),
            parent: Some("your parent".to_owned()),
            role: Some("your role".to_owned()),
        })
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}