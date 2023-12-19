#![allow(unused_imports)]
use asana::AsanaClient;
use asana::model::*;
#[tokio::main]
async fn main() {
    let client = AsanaClient::from_env();
    let data = CustomFieldRequest {
        custom_field_base: CustomFieldBase {
            custom_field_compact: CustomFieldCompact {
                asana_resource: AsanaResource {
                    gid: "your gid".to_owned(),
                    resource_type: "your resource type".to_owned(),
                },
                date_value: Some(serde_json::json!({})),
                display_value: Some("your display value".to_owned()),
                enabled: Some(true),
                enum_options: Some(
                    vec![
                        EnumOption { asana_resource : AsanaResource { gid : "your gid"
                        .to_owned(), resource_type : "your resource type".to_owned() },
                        color : Some("your color".to_owned()), enabled : Some(true), name
                        : Some("your name".to_owned()) }
                    ],
                ),
                enum_value: Some(serde_json::json!({})),
                is_formula_field: Some(true),
                multi_enum_values: Some(
                    vec![
                        EnumOption { asana_resource : AsanaResource { gid : "your gid"
                        .to_owned(), resource_type : "your resource type".to_owned() },
                        color : Some("your color".to_owned()), enabled : Some(true), name
                        : Some("your name".to_owned()) }
                    ],
                ),
                name: Some("your name".to_owned()),
                number_value: Some(1.0),
                resource_subtype: Some("your resource subtype".to_owned()),
                text_value: Some("your text value".to_owned()),
                type_: Some("your type".to_owned()),
            },
            asana_created_field: Some(serde_json::json!({})),
            currency_code: Some("your currency code".to_owned()),
            custom_label: Some("your custom label".to_owned()),
            custom_label_position: Some(serde_json::json!({})),
            description: Some("your description".to_owned()),
            enum_options: Some(
                vec![
                    EnumOption { asana_resource : AsanaResource { gid : "your gid"
                    .to_owned(), resource_type : "your resource type".to_owned() }, color
                    : Some("your color".to_owned()), enabled : Some(true), name :
                    Some("your name".to_owned()) }
                ],
            ),
            format: Some("your format".to_owned()),
            has_notifications_enabled: Some(true),
            is_global_to_workspace: Some(true),
            precision: Some(1),
        },
        owned_by_app: Some(true),
        people_value: Some(vec!["your people value".to_owned()]),
        workspace: "your workspace".to_owned(),
    };
    let response = client
        .create_custom_field(data)
        .opt_fields(&["your opt fields"])
        .opt_pretty(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}