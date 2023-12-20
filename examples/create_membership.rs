#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = CreateMembershipRequestBody {
        membership_request: MembershipRequest {
            is_active: true,
        },
        member: Some("your member".to_owned()),
        parent: Some("your parent".to_owned()),
        role: Some("your role".to_owned()),
    };
    let response = client.create_membership(data).opt_pretty(true).await.unwrap();
    println!("{:#?}", response);
}