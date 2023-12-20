//! [`AsanaClient`](struct.AsanaClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
pub use httpclient::{Error, Result, InMemoryResponseExt};
use crate::model::*;
mod serde;
#[derive(Clone)]
pub struct FluentRequest<'a, T> {
    pub(crate) client: &'a AsanaClient,
    pub params: T,
}
pub struct AsanaClient {
    pub client: httpclient::Client,
    authentication: AsanaAuthentication,
}
impl AsanaClient {
    pub fn from_env() -> Self {
        Self {
            client: httpclient::Client::new().base_url("https://app.asana.com/api/1.0"),
            authentication: AsanaAuthentication::from_env(),
        }
    }
}
impl AsanaClient {
    pub fn new(url: &str, authentication: AsanaAuthentication) -> Self {
        let client = httpclient::Client::new().base_url(url);
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: AsanaAuthentication) -> Self {
        self.authentication = authentication;
        self
    }
    pub(crate) fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            AsanaAuthentication::PersonalAccessToken { personal_access_token } => {
                r = r.bearer_auth(personal_access_token);
            }
        }
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    /**Get an attachment

Get the full record for a single attachment.*/
    pub fn get_attachment(
        &self,
        attachment_gid: &str,
    ) -> FluentRequest<'_, request::GetAttachmentRequest> {
        FluentRequest {
            client: self,
            params: request::GetAttachmentRequest {
                attachment_gid: attachment_gid.to_owned(),
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Delete an attachment

Deletes a specific, existing attachment.

Returns an empty data record.*/
    pub fn delete_attachment(
        &self,
        attachment_gid: &str,
    ) -> FluentRequest<'_, request::DeleteAttachmentRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteAttachmentRequest {
                attachment_gid: attachment_gid.to_owned(),
                opt_pretty: None,
            },
        }
    }
    /**Get attachments from an object

Returns the compact records for all attachments on the object.

There are three possible `parent` values for this request: `project`, `project_brief`, and `task`. For a project, an attachment refers to a file uploaded to the "Key resources" section in the project Overview. For a project brief, an attachment refers to inline files in the project brief itself. For a task, an attachment refers to a file directly associated to that task.

Note that within the Asana app, inline images in the task description do not appear in the index of image thumbnails nor as stories in the task. However, requests made to `GET /attachments` for a task will return all of the images in the task, including inline images.*/
    pub fn get_attachments_for_object(
        &self,
        parent: &str,
    ) -> FluentRequest<'_, request::GetAttachmentsForObjectRequest> {
        FluentRequest {
            client: self,
            params: request::GetAttachmentsForObjectRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                parent: parent.to_owned(),
            },
        }
    }
    /**Upload an attachment

Upload an attachment.

This method uploads an attachment on an object and returns the compact
record for the created attachment object. This is possible by either:

- Providing the URL of the external resource being attached, or
- Downloading the file content first and then uploading it as any other attachment. Note that it is not possible to attach
files from third party services such as Dropbox, Box, Vimeo & Google Drive via the API

The 100MB size limit on attachments in Asana is enforced on this endpoint.

This endpoint expects a multipart/form-data encoded request containing the full contents of the file to be uploaded.

Requests made should follow the HTTP/1.1 specification that line
terminators are of the form `CRLF` or `\r\n` outlined
[here](http://www.w3.org/Protocols/HTTP/1.1/draft-ietf-http-v11-spec-01#Basic-Rules) in order for the server to reliably and properly handle the request.*/
    pub fn create_attachment_for_object(
        &self,
    ) -> FluentRequest<'_, request::CreateAttachmentForObjectRequest> {
        FluentRequest {
            client: self,
            params: request::CreateAttachmentForObjectRequest {
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get audit log events

Retrieve the audit log events that have been captured in your domain.

This endpoint will return a list of [AuditLogEvent](/reference/audit-log-api) objects, sorted by creation time in ascending order. Note that the Audit Log API captures events from October 8th, 2021 and later. Queries for events before this date will not return results.

There are a number of query parameters (below) that can be used to filter the set of [AuditLogEvent](/reference/audit-log-api) objects that are returned in the response. Any combination of query parameters is valid. When no filters are provided, all of the events that have been captured in your domain will match.

The list of events will always be [paginated](/docs/pagination). The default limit is 1000 events. The next set of events can be retrieved using the `offset` from the previous response. If there are no events that match the provided filters in your domain, the endpoint will return `null` for the `next_page` field. Querying again with the same filters may return new events if they were captured after the last request. Once a response includes a `next_page` with an `offset`, subsequent requests can be made with the latest `offset` to poll for new events that match the provided filters.

*Note: If the filters you provided match events in your domain and `next_page` is present in the response, we will continue to send `next_page` on subsequent requests even when there are no more events that match the filters. This was put in place so that you can implement an audit log stream that will return future events that match these filters. If you are not interested in future events that match the filters you have defined, you can rely on checking empty `data` response for the end of current events that match your filters.*

When no `offset` is provided, the response will begin with the oldest events that match the provided filters. It is important to note that [AuditLogEvent](/reference/audit-log-api) objects will be permanently deleted from our systems after 90 days. If you wish to keep a permanent record of these events, we recommend using a SIEM tool to ingest and store these logs.*/
    pub fn get_audit_log_events(
        &self,
        workspace_gid: &str,
    ) -> FluentRequest<'_, request::GetAuditLogEventsRequest> {
        FluentRequest {
            client: self,
            params: request::GetAuditLogEventsRequest {
                actor_gid: None,
                actor_type: None,
                end_at: None,
                event_type: None,
                limit: None,
                offset: None,
                resource_gid: None,
                start_at: None,
                workspace_gid: workspace_gid.to_owned(),
            },
        }
    }
    /**Submit parallel requests

Make multiple requests in parallel to Asana's API.*/
    pub fn create_batch_request(
        &self,
        data: BatchRequest,
    ) -> FluentRequest<'_, request::CreateBatchRequestRequest> {
        FluentRequest {
            client: self,
            params: request::CreateBatchRequestRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get a project's custom fields

Returns a list of all of the custom fields settings on a project, in compact form. Note that, as in all queries to collections which return compact representation, `opt_fields` can be used to include more data than is returned in the compact representation. See the [documentation for input/output options](https://developers.asana.com/docs/inputoutput-options) for more information.*/
    pub fn get_custom_field_settings_for_project(
        &self,
        project_gid: &str,
    ) -> FluentRequest<'_, request::GetCustomFieldSettingsForProjectRequest> {
        FluentRequest {
            client: self,
            params: request::GetCustomFieldSettingsForProjectRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Get a portfolio's custom fields

Returns a list of all of the custom fields settings on a portfolio, in compact form.*/
    pub fn get_custom_field_settings_for_portfolio(
        &self,
        portfolio_gid: &str,
    ) -> FluentRequest<'_, request::GetCustomFieldSettingsForPortfolioRequest> {
        FluentRequest {
            client: self,
            params: request::GetCustomFieldSettingsForPortfolioRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                portfolio_gid: portfolio_gid.to_owned(),
            },
        }
    }
    /**Create a custom field

Creates a new custom field in a workspace. Every custom field is required
to be created in a specific workspace, and this workspace cannot be
changed once set.

A custom field’s name must be unique within a workspace and not conflict
with names of existing task properties such as `Due Date` or `Assignee`.
A custom field’s type must be one of `text`, `enum`, `multi_enum`, `number`,
`date`, or `people`.

Returns the full record of the newly created custom field.*/
    pub fn create_custom_field(
        &self,
        data: CustomFieldRequest,
    ) -> FluentRequest<'_, request::CreateCustomFieldRequest> {
        FluentRequest {
            client: self,
            params: request::CreateCustomFieldRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get a custom field

Get the complete definition of a custom field’s metadata.

Since custom fields can be defined for one of a number of types, and
these types have different data and behaviors, there are fields that are
relevant to a particular type. For instance, as noted above, enum_options
is only relevant for the enum type and defines the set of choices that
the enum could represent. The examples below show some of these
type-specific custom field definitions.*/
    pub fn get_custom_field(
        &self,
        custom_field_gid: &str,
    ) -> FluentRequest<'_, request::GetCustomFieldRequest> {
        FluentRequest {
            client: self,
            params: request::GetCustomFieldRequest {
                custom_field_gid: custom_field_gid.to_owned(),
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Update a custom field

A specific, existing custom field can be updated by making a PUT request on the URL for that custom field. Only the fields provided in the `data` block will be updated; any unspecified fields will remain unchanged
When using this method, it is best to specify only those fields you wish to change, or else you may overwrite changes made by another user since you last retrieved the custom field.
A custom field’s `type` cannot be updated.
An enum custom field’s `enum_options` cannot be updated with this endpoint. Instead see “Work With Enum Options” for information on how to update `enum_options`.
Locked custom fields can only be updated by the user who locked the field.
Returns the complete updated custom field record.*/
    pub fn update_custom_field(
        &self,
        custom_field_gid: &str,
        data: CustomFieldRequest,
    ) -> FluentRequest<'_, request::UpdateCustomFieldRequest> {
        FluentRequest {
            client: self,
            params: request::UpdateCustomFieldRequest {
                custom_field_gid: custom_field_gid.to_owned(),
                data,
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Delete a custom field

A specific, existing custom field can be deleted by making a DELETE request on the URL for that custom field.
Locked custom fields can only be deleted by the user who locked the field.
Returns an empty data record.*/
    pub fn delete_custom_field(
        &self,
        custom_field_gid: &str,
    ) -> FluentRequest<'_, request::DeleteCustomFieldRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteCustomFieldRequest {
                custom_field_gid: custom_field_gid.to_owned(),
                opt_pretty: None,
            },
        }
    }
    /**Get a workspace's custom fields

Returns a list of the compact representation of all of the custom fields in a workspace.*/
    pub fn get_custom_fields_for_workspace(
        &self,
        workspace_gid: &str,
    ) -> FluentRequest<'_, request::GetCustomFieldsForWorkspaceRequest> {
        FluentRequest {
            client: self,
            params: request::GetCustomFieldsForWorkspaceRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                workspace_gid: workspace_gid.to_owned(),
            },
        }
    }
    /**Create an enum option

Creates an enum option and adds it to this custom field’s list of enum options. A custom field can have at most 500 enum options (including disabled options). By default new enum options are inserted at the end of a custom field’s list.
Locked custom fields can only have enum options added by the user who locked the field.
Returns the full record of the newly created enum option.*/
    pub fn create_enum_option_for_custom_field(
        &self,
        custom_field_gid: &str,
        data: EnumOptionRequest,
    ) -> FluentRequest<'_, request::CreateEnumOptionForCustomFieldRequest> {
        FluentRequest {
            client: self,
            params: request::CreateEnumOptionForCustomFieldRequest {
                custom_field_gid: custom_field_gid.to_owned(),
                data,
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Reorder a custom field's enum

Moves a particular enum option to be either before or after another specified enum option in the custom field.
Locked custom fields can only be reordered by the user who locked the field.*/
    pub fn insert_enum_option_for_custom_field(
        &self,
        custom_field_gid: &str,
        data: EnumOptionInsertRequest,
    ) -> FluentRequest<'_, request::InsertEnumOptionForCustomFieldRequest> {
        FluentRequest {
            client: self,
            params: request::InsertEnumOptionForCustomFieldRequest {
                custom_field_gid: custom_field_gid.to_owned(),
                data,
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Update an enum option

Updates an existing enum option. Enum custom fields require at least one enabled enum option.
Locked custom fields can only be updated by the user who locked the field.
Returns the full record of the updated enum option.*/
    pub fn update_enum_option(
        &self,
        data: EnumOptionBase,
        enum_option_gid: &str,
    ) -> FluentRequest<'_, request::UpdateEnumOptionRequest> {
        FluentRequest {
            client: self,
            params: request::UpdateEnumOptionRequest {
                data,
                enum_option_gid: enum_option_gid.to_owned(),
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get events on a resource

Returns the full record for all events that have occurred since the sync
token was created.

A `GET` request to the endpoint `/[path_to_resource]/events` can be made in
lieu of including the resource ID in the data for the request.

Asana limits a single sync token to 100 events. If more than 100 events exist
for a given resource, `has_more: true` will be returned in the response, indicating
that there are more events to pull.

*Note: The resource returned will be the resource that triggered the
event. This may be different from the one that the events were requested
for. For example, a subscription to a project will contain events for
tasks contained within the project.**/
    pub fn get_events(
        &self,
        resource: &str,
    ) -> FluentRequest<'_, request::GetEventsRequest> {
        FluentRequest {
            client: self,
            params: request::GetEventsRequest {
                opt_fields: None,
                opt_pretty: None,
                resource: resource.to_owned(),
                sync: None,
            },
        }
    }
    /**Get a goal relationship

Returns the complete updated goal relationship record for a single goal relationship.*/
    pub fn get_goal_relationship(
        &self,
        goal_relationship_gid: &str,
    ) -> FluentRequest<'_, request::GetGoalRelationshipRequest> {
        FluentRequest {
            client: self,
            params: request::GetGoalRelationshipRequest {
                goal_relationship_gid: goal_relationship_gid.to_owned(),
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Update a goal relationship

An existing goal relationship can be updated by making a PUT request on the URL for
that goal relationship. Only the fields provided in the `data` block will be updated;
any unspecified fields will remain unchanged.

Returns the complete updated goal relationship record.*/
    pub fn update_goal_relationship(
        &self,
        data: GoalRelationshipRequest,
        goal_relationship_gid: &str,
    ) -> FluentRequest<'_, request::UpdateGoalRelationshipRequest> {
        FluentRequest {
            client: self,
            params: request::UpdateGoalRelationshipRequest {
                data,
                goal_relationship_gid: goal_relationship_gid.to_owned(),
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get goal relationships

Returns compact goal relationship records.*/
    pub fn get_goal_relationships(
        &self,
        supported_goal: &str,
    ) -> FluentRequest<'_, request::GetGoalRelationshipsRequest> {
        FluentRequest {
            client: self,
            params: request::GetGoalRelationshipsRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                resource_subtype: None,
                supported_goal: supported_goal.to_owned(),
            },
        }
    }
    /**Add a supporting goal relationship

Creates a goal relationship by adding a supporting resource to a given goal.

Returns the newly created goal relationship record.*/
    pub fn add_supporting_relationship(
        &self,
        data: GoalAddSupportingRelationshipRequest,
        goal_gid: &str,
    ) -> FluentRequest<'_, request::AddSupportingRelationshipRequest> {
        FluentRequest {
            client: self,
            params: request::AddSupportingRelationshipRequest {
                data,
                goal_gid: goal_gid.to_owned(),
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Removes a supporting goal relationship

Removes a goal relationship for a given parent goal.*/
    pub fn remove_supporting_relationship(
        &self,
        data: GoalRemoveSupportingRelationshipRequest,
        goal_gid: &str,
    ) -> FluentRequest<'_, request::RemoveSupportingRelationshipRequest> {
        FluentRequest {
            client: self,
            params: request::RemoveSupportingRelationshipRequest {
                data,
                goal_gid: goal_gid.to_owned(),
                opt_pretty: None,
            },
        }
    }
    /**Get a goal

Returns the complete goal record for a single goal.*/
    pub fn get_goal(
        &self,
        goal_gid: &str,
    ) -> FluentRequest<'_, request::GetGoalRequest> {
        FluentRequest {
            client: self,
            params: request::GetGoalRequest {
                goal_gid: goal_gid.to_owned(),
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Update a goal

An existing goal can be updated by making a PUT request on the URL for
that goal. Only the fields provided in the `data` block will be updated;
any unspecified fields will remain unchanged.

Returns the complete updated goal record.*/
    pub fn update_goal(
        &self,
        data: GoalUpdateRequest,
        goal_gid: &str,
    ) -> FluentRequest<'_, request::UpdateGoalRequest> {
        FluentRequest {
            client: self,
            params: request::UpdateGoalRequest {
                data,
                goal_gid: goal_gid.to_owned(),
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Delete a goal

A specific, existing goal can be deleted by making a DELETE request on the URL for that goal.

Returns an empty data record.*/
    pub fn delete_goal(
        &self,
        goal_gid: &str,
    ) -> FluentRequest<'_, request::DeleteGoalRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteGoalRequest {
                goal_gid: goal_gid.to_owned(),
                opt_pretty: None,
            },
        }
    }
    /**Get goals

Returns compact goal records.*/
    pub fn get_goals(&self) -> FluentRequest<'_, request::GetGoalsRequest> {
        FluentRequest {
            client: self,
            params: request::GetGoalsRequest {
                is_workspace_level: None,
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                portfolio: None,
                project: None,
                task: None,
                team: None,
                time_periods: None,
                workspace: None,
            },
        }
    }
    /**Create a goal

Creates a new goal in a workspace or team.

Returns the full record of the newly created goal.*/
    pub fn create_goal(
        &self,
        data: GoalRequest,
    ) -> FluentRequest<'_, request::CreateGoalRequest> {
        FluentRequest {
            client: self,
            params: request::CreateGoalRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Create a goal metric

Creates and adds a goal metric to a specified goal. Note that this replaces an existing goal metric if one already exists.*/
    pub fn create_goal_metric(
        &self,
        data: GoalMetricRequest,
        goal_gid: &str,
    ) -> FluentRequest<'_, request::CreateGoalMetricRequest> {
        FluentRequest {
            client: self,
            params: request::CreateGoalMetricRequest {
                data,
                goal_gid: goal_gid.to_owned(),
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Update a goal metric

Updates a goal's existing metric's `current_number_value` if one exists,
otherwise responds with a 400 status code.

Returns the complete updated goal metric record.*/
    pub fn update_goal_metric(
        &self,
        data: GoalMetricCurrentValueRequest,
        goal_gid: &str,
    ) -> FluentRequest<'_, request::UpdateGoalMetricRequest> {
        FluentRequest {
            client: self,
            params: request::UpdateGoalMetricRequest {
                data,
                goal_gid: goal_gid.to_owned(),
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Add a collaborator to a goal

Adds followers to a goal. Returns the goal the followers were added to.
Each goal can be associated with zero or more followers in the system.
Requests to add/remove followers, if successful, will return the complete updated goal record, described above.*/
    pub fn add_followers(
        &self,
        data: TaskAddFollowersRequest,
        goal_gid: &str,
    ) -> FluentRequest<'_, request::AddFollowersRequest> {
        FluentRequest {
            client: self,
            params: request::AddFollowersRequest {
                data,
                goal_gid: goal_gid.to_owned(),
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Remove a collaborator from a goal

Removes followers from a goal. Returns the goal the followers were removed from.
Each goal can be associated with zero or more followers in the system.
Requests to add/remove followers, if successful, will return the complete updated goal record, described above.*/
    pub fn remove_followers(
        &self,
        data: TaskAddFollowersRequest,
        goal_gid: &str,
    ) -> FluentRequest<'_, request::RemoveFollowersRequest> {
        FluentRequest {
            client: self,
            params: request::RemoveFollowersRequest {
                data,
                goal_gid: goal_gid.to_owned(),
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get parent goals from a goal

Returns a compact representation of all of the parent goals of a goal.*/
    pub fn get_parent_goals_for_goal(
        &self,
        goal_gid: &str,
    ) -> FluentRequest<'_, request::GetParentGoalsForGoalRequest> {
        FluentRequest {
            client: self,
            params: request::GetParentGoalsForGoalRequest {
                goal_gid: goal_gid.to_owned(),
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get a job by id

Returns the full record for a job.*/
    pub fn get_job(&self, job_gid: &str) -> FluentRequest<'_, request::GetJobRequest> {
        FluentRequest {
            client: self,
            params: request::GetJobRequest {
                job_gid: job_gid.to_owned(),
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get multiple memberships

Returns compact `goal_membership` or `project_membership` records. The possible types for `parent` in this request are `goal` or `project`. An additional member (user GID or team GID) can be passed in to filter to a specific membership.*/
    pub fn get_memberships(&self) -> FluentRequest<'_, request::GetMembershipsRequest> {
        FluentRequest {
            client: self,
            params: request::GetMembershipsRequest {
                limit: None,
                member: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                parent: None,
            },
        }
    }
    /**Create a membership

Creates a new membership in a `goal`. `Teams` or `users` can be a member
of `goals`.

Returns the full record of the newly created membership.*/
    pub fn create_membership(
        &self,
        data: CreateMembershipRequestBody,
    ) -> FluentRequest<'_, request::CreateMembershipRequest> {
        FluentRequest {
            client: self,
            params: request::CreateMembershipRequest {
                data,
                opt_pretty: None,
            },
        }
    }
    /**Get a membership

Returns compact `project_membership` record for a single membership. `GET` only supports project memberships currently*/
    pub fn get_membership(
        &self,
        membership_gid: &str,
    ) -> FluentRequest<'_, request::GetMembershipRequest> {
        FluentRequest {
            client: self,
            params: request::GetMembershipRequest {
                membership_gid: membership_gid.to_owned(),
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Delete a membership

A specific, existing membership can be deleted by making a `DELETE` request
on the URL for that membership.

Returns an empty data record.*/
    pub fn delete_membership(
        &self,
        membership_gid: &str,
    ) -> FluentRequest<'_, request::DeleteMembershipRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteMembershipRequest {
                membership_gid: membership_gid.to_owned(),
                opt_pretty: None,
            },
        }
    }
    /**Create an organization export request

This method creates a request to export an Organization. Asana will complete the export at some point after you create the request.*/
    pub fn create_organization_export(
        &self,
        data: OrganizationExportRequest,
    ) -> FluentRequest<'_, request::CreateOrganizationExportRequest> {
        FluentRequest {
            client: self,
            params: request::CreateOrganizationExportRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get details on an org export request

Returns details of a previously-requested Organization export.*/
    pub fn get_organization_export(
        &self,
        organization_export_gid: &str,
    ) -> FluentRequest<'_, request::GetOrganizationExportRequest> {
        FluentRequest {
            client: self,
            params: request::GetOrganizationExportRequest {
                opt_fields: None,
                opt_pretty: None,
                organization_export_gid: organization_export_gid.to_owned(),
            },
        }
    }
    /**Get multiple portfolio memberships

Returns a list of portfolio memberships in compact representation. You must specify `portfolio`, `portfolio` and `user`, or `workspace` and `user`.*/
    pub fn get_portfolio_memberships(
        &self,
    ) -> FluentRequest<'_, request::GetPortfolioMembershipsRequest> {
        FluentRequest {
            client: self,
            params: request::GetPortfolioMembershipsRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                portfolio: None,
                user: None,
                workspace: None,
            },
        }
    }
    /**Get a portfolio membership

Returns the complete portfolio record for a single portfolio membership.*/
    pub fn get_portfolio_membership(
        &self,
        portfolio_membership_gid: &str,
    ) -> FluentRequest<'_, request::GetPortfolioMembershipRequest> {
        FluentRequest {
            client: self,
            params: request::GetPortfolioMembershipRequest {
                opt_fields: None,
                opt_pretty: None,
                portfolio_membership_gid: portfolio_membership_gid.to_owned(),
            },
        }
    }
    /**Get memberships from a portfolio

Returns the compact portfolio membership records for the portfolio.*/
    pub fn get_portfolio_memberships_for_portfolio(
        &self,
        portfolio_gid: &str,
    ) -> FluentRequest<'_, request::GetPortfolioMembershipsForPortfolioRequest> {
        FluentRequest {
            client: self,
            params: request::GetPortfolioMembershipsForPortfolioRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                portfolio_gid: portfolio_gid.to_owned(),
                user: None,
            },
        }
    }
    /**Get multiple portfolios

Returns a list of the portfolios in compact representation that are owned by the current API user.*/
    pub fn get_portfolios(
        &self,
        workspace: &str,
    ) -> FluentRequest<'_, request::GetPortfoliosRequest> {
        FluentRequest {
            client: self,
            params: request::GetPortfoliosRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                owner: None,
                workspace: workspace.to_owned(),
            },
        }
    }
    /**Create a portfolio

Creates a new portfolio in the given workspace with the supplied name.

Note that portfolios created in the Asana UI may have some state
(like the “Priority” custom field) which is automatically added
to the portfolio when it is created. Portfolios created via our
API will *not* be created with the same initial state to allow
integrations to create their own starting state on a portfolio.*/
    pub fn create_portfolio(
        &self,
        data: PortfolioRequest,
    ) -> FluentRequest<'_, request::CreatePortfolioRequest> {
        FluentRequest {
            client: self,
            params: request::CreatePortfolioRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get a portfolio

Returns the complete portfolio record for a single portfolio.*/
    pub fn get_portfolio(
        &self,
        portfolio_gid: &str,
    ) -> FluentRequest<'_, request::GetPortfolioRequest> {
        FluentRequest {
            client: self,
            params: request::GetPortfolioRequest {
                opt_fields: None,
                opt_pretty: None,
                portfolio_gid: portfolio_gid.to_owned(),
            },
        }
    }
    /**Update a portfolio

An existing portfolio can be updated by making a PUT request on the URL for
that portfolio. Only the fields provided in the `data` block will be updated;
any unspecified fields will remain unchanged.

Returns the complete updated portfolio record.*/
    pub fn update_portfolio(
        &self,
        data: PortfolioRequest,
        portfolio_gid: &str,
    ) -> FluentRequest<'_, request::UpdatePortfolioRequest> {
        FluentRequest {
            client: self,
            params: request::UpdatePortfolioRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                portfolio_gid: portfolio_gid.to_owned(),
            },
        }
    }
    /**Delete a portfolio

An existing portfolio can be deleted by making a DELETE request on
the URL for that portfolio.

Returns an empty data record.*/
    pub fn delete_portfolio(
        &self,
        portfolio_gid: &str,
    ) -> FluentRequest<'_, request::DeletePortfolioRequest> {
        FluentRequest {
            client: self,
            params: request::DeletePortfolioRequest {
                opt_pretty: None,
                portfolio_gid: portfolio_gid.to_owned(),
            },
        }
    }
    /**Get portfolio items

Get a list of the items in compact form in a portfolio.*/
    pub fn get_items_for_portfolio(
        &self,
        portfolio_gid: &str,
    ) -> FluentRequest<'_, request::GetItemsForPortfolioRequest> {
        FluentRequest {
            client: self,
            params: request::GetItemsForPortfolioRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                portfolio_gid: portfolio_gid.to_owned(),
            },
        }
    }
    /**Add a portfolio item

Add an item to a portfolio.
Returns an empty data block.*/
    pub fn add_item_for_portfolio(
        &self,
        data: PortfolioAddItemRequest,
        portfolio_gid: &str,
    ) -> FluentRequest<'_, request::AddItemForPortfolioRequest> {
        FluentRequest {
            client: self,
            params: request::AddItemForPortfolioRequest {
                data,
                opt_pretty: None,
                portfolio_gid: portfolio_gid.to_owned(),
            },
        }
    }
    /**Remove a portfolio item

Remove an item from a portfolio.
Returns an empty data block.*/
    pub fn remove_item_for_portfolio(
        &self,
        data: PortfolioRemoveItemRequest,
        portfolio_gid: &str,
    ) -> FluentRequest<'_, request::RemoveItemForPortfolioRequest> {
        FluentRequest {
            client: self,
            params: request::RemoveItemForPortfolioRequest {
                data,
                opt_pretty: None,
                portfolio_gid: portfolio_gid.to_owned(),
            },
        }
    }
    /**Add a custom field to a portfolio

Custom fields are associated with portfolios by way of custom field settings.  This method creates a setting for the portfolio.*/
    pub fn add_custom_field_setting_for_portfolio(
        &self,
        data: AddCustomFieldSettingRequest,
        portfolio_gid: &str,
    ) -> FluentRequest<'_, request::AddCustomFieldSettingForPortfolioRequest> {
        FluentRequest {
            client: self,
            params: request::AddCustomFieldSettingForPortfolioRequest {
                data,
                opt_pretty: None,
                portfolio_gid: portfolio_gid.to_owned(),
            },
        }
    }
    /**Remove a custom field from a portfolio

Removes a custom field setting from a portfolio.*/
    pub fn remove_custom_field_setting_for_portfolio(
        &self,
        data: RemoveCustomFieldSettingRequest,
        portfolio_gid: &str,
    ) -> FluentRequest<'_, request::RemoveCustomFieldSettingForPortfolioRequest> {
        FluentRequest {
            client: self,
            params: request::RemoveCustomFieldSettingForPortfolioRequest {
                data,
                opt_pretty: None,
                portfolio_gid: portfolio_gid.to_owned(),
            },
        }
    }
    /**Add users to a portfolio

Adds the specified list of users as members of the portfolio.
Returns the updated portfolio record.*/
    pub fn add_members_for_portfolio(
        &self,
        data: AddMembersRequest,
        portfolio_gid: &str,
    ) -> FluentRequest<'_, request::AddMembersForPortfolioRequest> {
        FluentRequest {
            client: self,
            params: request::AddMembersForPortfolioRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                portfolio_gid: portfolio_gid.to_owned(),
            },
        }
    }
    /**Remove users from a portfolio

Removes the specified list of users from members of the portfolio.
Returns the updated portfolio record.*/
    pub fn remove_members_for_portfolio(
        &self,
        data: RemoveMembersRequest,
        portfolio_gid: &str,
    ) -> FluentRequest<'_, request::RemoveMembersForPortfolioRequest> {
        FluentRequest {
            client: self,
            params: request::RemoveMembersForPortfolioRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                portfolio_gid: portfolio_gid.to_owned(),
            },
        }
    }
    /**Get a project brief

Get the full record for a project brief.*/
    pub fn get_project_brief(
        &self,
        project_brief_gid: &str,
    ) -> FluentRequest<'_, request::GetProjectBriefRequest> {
        FluentRequest {
            client: self,
            params: request::GetProjectBriefRequest {
                opt_fields: None,
                opt_pretty: None,
                project_brief_gid: project_brief_gid.to_owned(),
            },
        }
    }
    /**Update a project brief

An existing project brief can be updated by making a PUT request on the URL for
that project brief. Only the fields provided in the `data` block will be updated;
any unspecified fields will remain unchanged.

Returns the complete updated project brief record.*/
    pub fn update_project_brief(
        &self,
        data: ProjectBriefRequest,
        project_brief_gid: &str,
    ) -> FluentRequest<'_, request::UpdateProjectBriefRequest> {
        FluentRequest {
            client: self,
            params: request::UpdateProjectBriefRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                project_brief_gid: project_brief_gid.to_owned(),
            },
        }
    }
    /**Delete a project brief

Deletes a specific, existing project brief.

Returns an empty data record.*/
    pub fn delete_project_brief(
        &self,
        project_brief_gid: &str,
    ) -> FluentRequest<'_, request::DeleteProjectBriefRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteProjectBriefRequest {
                opt_pretty: None,
                project_brief_gid: project_brief_gid.to_owned(),
            },
        }
    }
    /**Create a project brief

Creates a new project brief.

Returns the full record of the newly created project brief.*/
    pub fn create_project_brief(
        &self,
        data: ProjectBriefRequest,
        project_gid: &str,
    ) -> FluentRequest<'_, request::CreateProjectBriefRequest> {
        FluentRequest {
            client: self,
            params: request::CreateProjectBriefRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Get a project membership

Returns the complete project record for a single project membership.*/
    pub fn get_project_membership(
        &self,
        project_membership_gid: &str,
    ) -> FluentRequest<'_, request::GetProjectMembershipRequest> {
        FluentRequest {
            client: self,
            params: request::GetProjectMembershipRequest {
                opt_fields: None,
                opt_pretty: None,
                project_membership_gid: project_membership_gid.to_owned(),
            },
        }
    }
    /**Get memberships from a project

Returns the compact project membership records for the project.*/
    pub fn get_project_memberships_for_project(
        &self,
        project_gid: &str,
    ) -> FluentRequest<'_, request::GetProjectMembershipsForProjectRequest> {
        FluentRequest {
            client: self,
            params: request::GetProjectMembershipsForProjectRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
                user: None,
            },
        }
    }
    /**Get a project status

*Deprecated: new integrations should prefer the `/status_updates/{status_gid}` route.*

Returns the complete record for a single status update.*/
    pub fn get_project_status(
        &self,
        project_status_gid: &str,
    ) -> FluentRequest<'_, request::GetProjectStatusRequest> {
        FluentRequest {
            client: self,
            params: request::GetProjectStatusRequest {
                opt_fields: None,
                opt_pretty: None,
                project_status_gid: project_status_gid.to_owned(),
            },
        }
    }
    /**Delete a project status

*Deprecated: new integrations should prefer the `/status_updates/{status_gid}` route.*

Deletes a specific, existing project status update.

Returns an empty data record.*/
    pub fn delete_project_status(
        &self,
        project_status_gid: &str,
    ) -> FluentRequest<'_, request::DeleteProjectStatusRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteProjectStatusRequest {
                opt_pretty: None,
                project_status_gid: project_status_gid.to_owned(),
            },
        }
    }
    /**Get statuses from a project

*Deprecated: new integrations should prefer the `/status_updates` route.*

Returns the compact project status update records for all updates on the project.*/
    pub fn get_project_statuses_for_project(
        &self,
        project_gid: &str,
    ) -> FluentRequest<'_, request::GetProjectStatusesForProjectRequest> {
        FluentRequest {
            client: self,
            params: request::GetProjectStatusesForProjectRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Create a project status

*Deprecated: new integrations should prefer the `/status_updates` route.*

Creates a new status update on the project.

Returns the full record of the newly created project status update.*/
    pub fn create_project_status_for_project(
        &self,
        data: ProjectStatusRequest,
        project_gid: &str,
    ) -> FluentRequest<'_, request::CreateProjectStatusForProjectRequest> {
        FluentRequest {
            client: self,
            params: request::CreateProjectStatusForProjectRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Get a project template

Returns the complete project template record for a single project template.*/
    pub fn get_project_template(
        &self,
        project_template_gid: &str,
    ) -> FluentRequest<'_, request::GetProjectTemplateRequest> {
        FluentRequest {
            client: self,
            params: request::GetProjectTemplateRequest {
                opt_fields: None,
                opt_pretty: None,
                project_template_gid: project_template_gid.to_owned(),
            },
        }
    }
    /**Delete a project template

A specific, existing project template can be deleted by making a DELETE request on the URL for that project template.

Returns an empty data record.*/
    pub fn delete_project_template(
        &self,
        project_template_gid: &str,
    ) -> FluentRequest<'_, request::DeleteProjectTemplateRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteProjectTemplateRequest {
                opt_pretty: None,
                project_template_gid: project_template_gid.to_owned(),
            },
        }
    }
    /**Get multiple project templates

Returns the compact project template records for all project templates in the given team or workspace.*/
    pub fn get_project_templates(
        &self,
    ) -> FluentRequest<'_, request::GetProjectTemplatesRequest> {
        FluentRequest {
            client: self,
            params: request::GetProjectTemplatesRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                team: None,
                workspace: None,
            },
        }
    }
    /**Get a team's project templates

Returns the compact project template records for all project templates in the team.*/
    pub fn get_project_templates_for_team(
        &self,
        team_gid: &str,
    ) -> FluentRequest<'_, request::GetProjectTemplatesForTeamRequest> {
        FluentRequest {
            client: self,
            params: request::GetProjectTemplatesForTeamRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                team_gid: team_gid.to_owned(),
            },
        }
    }
    /**Instantiate a project from a project template

Creates and returns a job that will asynchronously handle the project instantiation.

To form this request, it is recommended to first make a request to [get a project template](/reference/getprojecttemplate). Then, from the response, copy the `gid` from the object in the `requested_dates` array. This `gid` should be used in `requested_dates` to instantiate a project.

_Note: The body of this request will differ if your workspace is an organization. To determine if your workspace is an organization, use the [is_organization](/reference/workspaces) parameter._*/
    pub fn instantiate_project(
        &self,
        data: ProjectTemplateInstantiateProjectRequest,
        project_template_gid: &str,
    ) -> FluentRequest<'_, request::InstantiateProjectRequest> {
        FluentRequest {
            client: self,
            params: request::InstantiateProjectRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                project_template_gid: project_template_gid.to_owned(),
            },
        }
    }
    /**Get multiple projects

Returns the compact project records for some filtered set of projects. Use one or more of the parameters provided to filter the projects returned.
*Note: This endpoint may timeout for large domains. Try filtering by team!**/
    pub fn get_projects(&self) -> FluentRequest<'_, request::GetProjectsRequest> {
        FluentRequest {
            client: self,
            params: request::GetProjectsRequest {
                archived: None,
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                team: None,
                workspace: None,
            },
        }
    }
    /**Create a project

Create a new project in a workspace or team.

Every project is required to be created in a specific workspace or
organization, and this cannot be changed once set. Note that you can use
the `workspace` parameter regardless of whether or not it is an
organization.

If the workspace for your project is an organization, you must also
supply a `team` to share the project with.

Returns the full record of the newly created project.*/
    pub fn create_project(
        &self,
        data: ProjectRequest,
    ) -> FluentRequest<'_, request::CreateProjectRequest> {
        FluentRequest {
            client: self,
            params: request::CreateProjectRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get a project

Returns the complete project record for a single project.*/
    pub fn get_project(
        &self,
        project_gid: &str,
    ) -> FluentRequest<'_, request::GetProjectRequest> {
        FluentRequest {
            client: self,
            params: request::GetProjectRequest {
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Update a project

A specific, existing project can be updated by making a PUT request on
the URL for that project. Only the fields provided in the `data` block
will be updated; any unspecified fields will remain unchanged.

When using this method, it is best to specify only those fields you wish
to change, or else you may overwrite changes made by another user since
you last retrieved the task.

Returns the complete updated project record.*/
    pub fn update_project(
        &self,
        data: ProjectUpdateRequest,
        project_gid: &str,
    ) -> FluentRequest<'_, request::UpdateProjectRequest> {
        FluentRequest {
            client: self,
            params: request::UpdateProjectRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Delete a project

A specific, existing project can be deleted by making a DELETE request on
the URL for that project.

Returns an empty data record.*/
    pub fn delete_project(
        &self,
        project_gid: &str,
    ) -> FluentRequest<'_, request::DeleteProjectRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteProjectRequest {
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Duplicate a project

Creates and returns a job that will asynchronously handle the duplication.*/
    pub fn duplicate_project(
        &self,
        data: ProjectDuplicateRequest,
        project_gid: &str,
    ) -> FluentRequest<'_, request::DuplicateProjectRequest> {
        FluentRequest {
            client: self,
            params: request::DuplicateProjectRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Get projects a task is in

Returns a compact representation of all of the projects the task is in.*/
    pub fn get_projects_for_task(
        &self,
        task_gid: &str,
    ) -> FluentRequest<'_, request::GetProjectsForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::GetProjectsForTaskRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Get a team's projects

Returns the compact project records for all projects in the team.*/
    pub fn get_projects_for_team(
        &self,
        team_gid: &str,
    ) -> FluentRequest<'_, request::GetProjectsForTeamRequest> {
        FluentRequest {
            client: self,
            params: request::GetProjectsForTeamRequest {
                archived: None,
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                team_gid: team_gid.to_owned(),
            },
        }
    }
    /**Create a project in a team

Creates a project shared with the given team.

Returns the full record of the newly created project.*/
    pub fn create_project_for_team(
        &self,
        data: ProjectRequest,
        team_gid: &str,
    ) -> FluentRequest<'_, request::CreateProjectForTeamRequest> {
        FluentRequest {
            client: self,
            params: request::CreateProjectForTeamRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                team_gid: team_gid.to_owned(),
            },
        }
    }
    /**Get all projects in a workspace

Returns the compact project records for all projects in the workspace.
*Note: This endpoint may timeout for large domains. Prefer the `/teams/{team_gid}/projects` endpoint.**/
    pub fn get_projects_for_workspace(
        &self,
        workspace_gid: &str,
    ) -> FluentRequest<'_, request::GetProjectsForWorkspaceRequest> {
        FluentRequest {
            client: self,
            params: request::GetProjectsForWorkspaceRequest {
                archived: None,
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                workspace_gid: workspace_gid.to_owned(),
            },
        }
    }
    /**Create a project in a workspace

Creates a project in the workspace.

If the workspace for your project is an organization, you must also
supply a team to share the project with.

Returns the full record of the newly created project.*/
    pub fn create_project_for_workspace(
        &self,
        data: ProjectRequest,
        workspace_gid: &str,
    ) -> FluentRequest<'_, request::CreateProjectForWorkspaceRequest> {
        FluentRequest {
            client: self,
            params: request::CreateProjectForWorkspaceRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                workspace_gid: workspace_gid.to_owned(),
            },
        }
    }
    /**Add a custom field to a project

Custom fields are associated with projects by way of custom field settings.  This method creates a setting for the project.*/
    pub fn add_custom_field_setting_for_project(
        &self,
        data: AddCustomFieldSettingRequest,
        project_gid: &str,
    ) -> FluentRequest<'_, request::AddCustomFieldSettingForProjectRequest> {
        FluentRequest {
            client: self,
            params: request::AddCustomFieldSettingForProjectRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Remove a custom field from a project

Removes a custom field setting from a project.*/
    pub fn remove_custom_field_setting_for_project(
        &self,
        data: RemoveCustomFieldSettingRequest,
        project_gid: &str,
    ) -> FluentRequest<'_, request::RemoveCustomFieldSettingForProjectRequest> {
        FluentRequest {
            client: self,
            params: request::RemoveCustomFieldSettingForProjectRequest {
                data,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Get task count of a project

Get an object that holds task count fields. **All fields are excluded by default**. You must [opt in](/docs/inputoutput-options) using `opt_fields` to get any information from this endpoint.

This endpoint has an additional [rate limit](/docs/rate-limits) and each field counts especially high against our [cost limits](/docs/rate-limits#cost-limits).

Milestones are just tasks, so they are included in the `num_tasks`, `num_incomplete_tasks`, and `num_completed_tasks` counts.*/
    pub fn get_task_counts_for_project(
        &self,
        project_gid: &str,
    ) -> FluentRequest<'_, request::GetTaskCountsForProjectRequest> {
        FluentRequest {
            client: self,
            params: request::GetTaskCountsForProjectRequest {
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Add users to a project

Adds the specified list of users as members of the project. Note that a user being added as a member may also be added as a *follower* as a result of this operation. This is because the user's default notification settings (i.e., in the "Notifcations" tab of "My Profile Settings") will override this endpoint's default behavior of setting "Tasks added" notifications to `false`.
Returns the updated project record.*/
    pub fn add_members_for_project(
        &self,
        data: AddMembersRequest,
        project_gid: &str,
    ) -> FluentRequest<'_, request::AddMembersForProjectRequest> {
        FluentRequest {
            client: self,
            params: request::AddMembersForProjectRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Remove users from a project

Removes the specified list of users from members of the project.
Returns the updated project record.*/
    pub fn remove_members_for_project(
        &self,
        data: RemoveMembersRequest,
        project_gid: &str,
    ) -> FluentRequest<'_, request::RemoveMembersForProjectRequest> {
        FluentRequest {
            client: self,
            params: request::RemoveMembersForProjectRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Add followers to a project

Adds the specified list of users as followers to the project. Followers are a subset of members who have opted in to receive "tasks added" notifications for a project. Therefore, if the users are not already members of the project, they will also become members as a result of this operation.
Returns the updated project record.*/
    pub fn add_followers_for_project(
        &self,
        data: AddFollowersRequest,
        project_gid: &str,
    ) -> FluentRequest<'_, request::AddFollowersForProjectRequest> {
        FluentRequest {
            client: self,
            params: request::AddFollowersForProjectRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Remove followers from a project

Removes the specified list of users from following the project, this will not affect project membership status.
Returns the updated project record.*/
    pub fn remove_followers_for_project(
        &self,
        data: RemoveFollowersRequest,
        project_gid: &str,
    ) -> FluentRequest<'_, request::RemoveFollowersForProjectRequest> {
        FluentRequest {
            client: self,
            params: request::RemoveFollowersForProjectRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Create a project template from a project

Creates and returns a job that will asynchronously handle the project template creation. Note that
while the resulting project template can be accessed with the API, it won't be visible in the Asana
UI until Project Templates 2.0 is launched in the app. See more in [this forum post](https://forum.asana.com/t/a-new-api-for-project-templates/156432).*/
    pub fn project_save_as_template(
        &self,
        data: ProjectSaveAsTemplateRequestBody,
        project_gid: &str,
    ) -> FluentRequest<'_, request::ProjectSaveAsTemplateRequest> {
        FluentRequest {
            client: self,
            params: request::ProjectSaveAsTemplateRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Trigger a rule

Trigger a rule which uses an ["incoming web request"](/docs/incoming-web-requests) trigger.*/
    pub fn trigger_rule(
        &self,
        data: RuleTriggerRequest,
        rule_trigger_gid: &str,
    ) -> FluentRequest<'_, request::TriggerRuleRequest> {
        FluentRequest {
            client: self,
            params: request::TriggerRuleRequest {
                data,
                rule_trigger_gid: rule_trigger_gid.to_owned(),
            },
        }
    }
    /**Get a section

Returns the complete record for a single section.*/
    pub fn get_section(
        &self,
        section_gid: &str,
    ) -> FluentRequest<'_, request::GetSectionRequest> {
        FluentRequest {
            client: self,
            params: request::GetSectionRequest {
                opt_fields: None,
                opt_pretty: None,
                section_gid: section_gid.to_owned(),
            },
        }
    }
    /**Update a section

A specific, existing section can be updated by making a PUT request on
the URL for that project. Only the fields provided in the `data` block
will be updated; any unspecified fields will remain unchanged. (note that
at this time, the only field that can be updated is the `name` field.)

When using this method, it is best to specify only those fields you wish
to change, or else you may overwrite changes made by another user since
you last retrieved the task.

Returns the complete updated section record.*/
    pub fn update_section(
        &self,
        data: SectionRequest,
        section_gid: &str,
    ) -> FluentRequest<'_, request::UpdateSectionRequest> {
        FluentRequest {
            client: self,
            params: request::UpdateSectionRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                section_gid: section_gid.to_owned(),
            },
        }
    }
    /**Delete a section

A specific, existing section can be deleted by making a DELETE request on
the URL for that section.

Note that sections must be empty to be deleted.

The last remaining section cannot be deleted.

Returns an empty data block.*/
    pub fn delete_section(
        &self,
        section_gid: &str,
    ) -> FluentRequest<'_, request::DeleteSectionRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteSectionRequest {
                opt_pretty: None,
                section_gid: section_gid.to_owned(),
            },
        }
    }
    /**Get sections in a project

Returns the compact records for all sections in the specified project.*/
    pub fn get_sections_for_project(
        &self,
        project_gid: &str,
    ) -> FluentRequest<'_, request::GetSectionsForProjectRequest> {
        FluentRequest {
            client: self,
            params: request::GetSectionsForProjectRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Create a section in a project

Creates a new section in a project.
Returns the full record of the newly created section.*/
    pub fn create_section_for_project(
        &self,
        data: SectionRequest,
        project_gid: &str,
    ) -> FluentRequest<'_, request::CreateSectionForProjectRequest> {
        FluentRequest {
            client: self,
            params: request::CreateSectionForProjectRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Add task to section

Add a task to a specific, existing section. This will remove the task from other sections of the project.

The task will be inserted at the top of a section unless an insert_before or insert_after parameter is declared.

This does not work for separators (tasks with the resource_subtype of section).*/
    pub fn add_task_for_section(
        &self,
        data: SectionTaskInsertRequest,
        section_gid: &str,
    ) -> FluentRequest<'_, request::AddTaskForSectionRequest> {
        FluentRequest {
            client: self,
            params: request::AddTaskForSectionRequest {
                data,
                opt_pretty: None,
                section_gid: section_gid.to_owned(),
            },
        }
    }
    /**Move or Insert sections

Move sections relative to each other. One of
`before_section` or `after_section` is required.

Sections cannot be moved between projects.

Returns an empty data block.*/
    pub fn insert_section_for_project(
        &self,
        data: ProjectSectionInsertRequest,
        project_gid: &str,
    ) -> FluentRequest<'_, request::InsertSectionForProjectRequest> {
        FluentRequest {
            client: self,
            params: request::InsertSectionForProjectRequest {
                data,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Get a status update

Returns the complete record for a single status update.*/
    pub fn get_status(
        &self,
        status_update_gid: &str,
    ) -> FluentRequest<'_, request::GetStatusRequest> {
        FluentRequest {
            client: self,
            params: request::GetStatusRequest {
                opt_fields: None,
                opt_pretty: None,
                status_update_gid: status_update_gid.to_owned(),
            },
        }
    }
    /**Delete a status update

Deletes a specific, existing status update.

Returns an empty data record.*/
    pub fn delete_status(
        &self,
        status_update_gid: &str,
    ) -> FluentRequest<'_, request::DeleteStatusRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteStatusRequest {
                opt_pretty: None,
                status_update_gid: status_update_gid.to_owned(),
            },
        }
    }
    /**Get status updates from an object

Returns the compact status update records for all updates on the object.*/
    pub fn get_statuses_for_object(
        &self,
        parent: &str,
    ) -> FluentRequest<'_, request::GetStatusesForObjectRequest> {
        FluentRequest {
            client: self,
            params: request::GetStatusesForObjectRequest {
                created_since: None,
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                parent: parent.to_owned(),
            },
        }
    }
    /**Create a status update

Creates a new status update on an object.
Returns the full record of the newly created status update.*/
    pub fn create_status_for_object(
        &self,
        data: StatusUpdateRequest,
    ) -> FluentRequest<'_, request::CreateStatusForObjectRequest> {
        FluentRequest {
            client: self,
            params: request::CreateStatusForObjectRequest {
                data,
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get a story

Returns the full record for a single story.*/
    pub fn get_story(
        &self,
        story_gid: &str,
    ) -> FluentRequest<'_, request::GetStoryRequest> {
        FluentRequest {
            client: self,
            params: request::GetStoryRequest {
                opt_fields: None,
                opt_pretty: None,
                story_gid: story_gid.to_owned(),
            },
        }
    }
    /**Update a story

Updates the story and returns the full record for the updated story. Only comment stories can have their text updated, and only comment stories and attachment stories can be pinned. Only one of `text` and `html_text` can be specified.*/
    pub fn update_story(
        &self,
        data: StoryRequest,
        story_gid: &str,
    ) -> FluentRequest<'_, request::UpdateStoryRequest> {
        FluentRequest {
            client: self,
            params: request::UpdateStoryRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                story_gid: story_gid.to_owned(),
            },
        }
    }
    /**Delete a story

Deletes a story. A user can only delete stories they have created.

Returns an empty data record.*/
    pub fn delete_story(
        &self,
        story_gid: &str,
    ) -> FluentRequest<'_, request::DeleteStoryRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteStoryRequest {
                opt_pretty: None,
                story_gid: story_gid.to_owned(),
            },
        }
    }
    /**Get stories from a task

Returns the compact records for all stories on the task.*/
    pub fn get_stories_for_task(
        &self,
        task_gid: &str,
    ) -> FluentRequest<'_, request::GetStoriesForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::GetStoriesForTaskRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Create a story on a task

Adds a story to a task. This endpoint currently only allows for comment
stories to be created. The comment will be authored by the currently
authenticated user, and timestamped when the server receives the request.

Returns the full record for the new story added to the task.*/
    pub fn create_story_for_task(
        &self,
        data: StoryRequest,
        task_gid: &str,
    ) -> FluentRequest<'_, request::CreateStoryForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::CreateStoryForTaskRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Get multiple tags

Returns the compact tag records for some filtered set of tags. Use one or more of the parameters provided to filter the tags returned.*/
    pub fn get_tags(&self) -> FluentRequest<'_, request::GetTagsRequest> {
        FluentRequest {
            client: self,
            params: request::GetTagsRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                workspace: None,
            },
        }
    }
    /**Create a tag

Creates a new tag in a workspace or organization.

Every tag is required to be created in a specific workspace or
organization, and this cannot be changed once set. Note that you can use
the workspace parameter regardless of whether or not it is an
organization.

Returns the full record of the newly created tag.*/
    pub fn create_tag(
        &self,
        data: TagRequest,
    ) -> FluentRequest<'_, request::CreateTagRequest> {
        FluentRequest {
            client: self,
            params: request::CreateTagRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get a tag

Returns the complete tag record for a single tag.*/
    pub fn get_tag(&self, tag_gid: &str) -> FluentRequest<'_, request::GetTagRequest> {
        FluentRequest {
            client: self,
            params: request::GetTagRequest {
                opt_fields: None,
                opt_pretty: None,
                tag_gid: tag_gid.to_owned(),
            },
        }
    }
    /**Update a tag

Updates the properties of a tag. Only the fields provided in the `data`
block will be updated; any unspecified fields will remain unchanged.

When using this method, it is best to specify only those fields you wish
to change, or else you may overwrite changes made by another user since
you last retrieved the tag.

Returns the complete updated tag record.*/
    pub fn update_tag(
        &self,
        tag_gid: &str,
    ) -> FluentRequest<'_, request::UpdateTagRequest> {
        FluentRequest {
            client: self,
            params: request::UpdateTagRequest {
                opt_fields: None,
                opt_pretty: None,
                tag_gid: tag_gid.to_owned(),
            },
        }
    }
    /**Delete a tag

A specific, existing tag can be deleted by making a DELETE request on
the URL for that tag.

Returns an empty data record.*/
    pub fn delete_tag(
        &self,
        tag_gid: &str,
    ) -> FluentRequest<'_, request::DeleteTagRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteTagRequest {
                opt_pretty: None,
                tag_gid: tag_gid.to_owned(),
            },
        }
    }
    /**Get a task's tags

Get a compact representation of all of the tags the task has.*/
    pub fn get_tags_for_task(
        &self,
        task_gid: &str,
    ) -> FluentRequest<'_, request::GetTagsForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::GetTagsForTaskRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Get tags in a workspace

Returns the compact tag records for some filtered set of tags. Use one or more of the parameters provided to filter the tags returned.*/
    pub fn get_tags_for_workspace(
        &self,
        workspace_gid: &str,
    ) -> FluentRequest<'_, request::GetTagsForWorkspaceRequest> {
        FluentRequest {
            client: self,
            params: request::GetTagsForWorkspaceRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                workspace_gid: workspace_gid.to_owned(),
            },
        }
    }
    /**Create a tag in a workspace

Creates a new tag in a workspace or organization.

Every tag is required to be created in a specific workspace or
organization, and this cannot be changed once set. Note that you can use
the workspace parameter regardless of whether or not it is an
organization.

Returns the full record of the newly created tag.*/
    pub fn create_tag_for_workspace(
        &self,
        data: TagCreateTagForWorkspaceRequest,
        workspace_gid: &str,
    ) -> FluentRequest<'_, request::CreateTagForWorkspaceRequest> {
        FluentRequest {
            client: self,
            params: request::CreateTagForWorkspaceRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                workspace_gid: workspace_gid.to_owned(),
            },
        }
    }
    /**Get multiple task templates

Returns the compact task template records for some filtered set of task templates. You must specify a `project`*/
    pub fn get_task_templates(
        &self,
    ) -> FluentRequest<'_, request::GetTaskTemplatesRequest> {
        FluentRequest {
            client: self,
            params: request::GetTaskTemplatesRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                project: None,
            },
        }
    }
    /**Get a task template

Returns the complete task template record for a single task template.*/
    pub fn get_task_template(
        &self,
        task_template_gid: &str,
    ) -> FluentRequest<'_, request::GetTaskTemplateRequest> {
        FluentRequest {
            client: self,
            params: request::GetTaskTemplateRequest {
                opt_fields: None,
                opt_pretty: None,
                task_template_gid: task_template_gid.to_owned(),
            },
        }
    }
    /**Instantiate a task from a task template

Creates and returns a job that will asynchronously handle the task instantiation.*/
    pub fn instantiate_task(
        &self,
        data: TaskTemplateInstantiateTaskRequest,
        task_template_gid: &str,
    ) -> FluentRequest<'_, request::InstantiateTaskRequest> {
        FluentRequest {
            client: self,
            params: request::InstantiateTaskRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                task_template_gid: task_template_gid.to_owned(),
            },
        }
    }
    /**Get multiple tasks

Returns the compact task records for some filtered set of tasks. Use one or more of the parameters provided to filter the tasks returned. You must specify a `project` or `tag` if you do not specify `assignee` and `workspace`.

For more complex task retrieval, use [workspaces/{workspace_gid}/tasks/search](/reference/searchtasksforworkspace).*/
    pub fn get_tasks(&self) -> FluentRequest<'_, request::GetTasksRequest> {
        FluentRequest {
            client: self,
            params: request::GetTasksRequest {
                assignee: None,
                completed_since: None,
                limit: None,
                modified_since: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                project: None,
                section: None,
                workspace: None,
            },
        }
    }
    /**Create a task

Creating a new task is as easy as POSTing to the `/tasks` endpoint with a
data block containing the fields you’d like to set on the task. Any
unspecified fields will take on default values.

Every task is required to be created in a specific workspace, and this
workspace cannot be changed once set. The workspace need not be set
explicitly if you specify `projects` or a `parent` task instead.*/
    pub fn create_task(
        &self,
        data: TaskRequest,
    ) -> FluentRequest<'_, request::CreateTaskRequest> {
        FluentRequest {
            client: self,
            params: request::CreateTaskRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get a task

Returns the complete task record for a single task.*/
    pub fn get_task(
        &self,
        task_gid: &str,
    ) -> FluentRequest<'_, request::GetTaskRequest> {
        FluentRequest {
            client: self,
            params: request::GetTaskRequest {
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Update a task

A specific, existing task can be updated by making a PUT request on the
URL for that task. Only the fields provided in the `data` block will be
updated; any unspecified fields will remain unchanged.

When using this method, it is best to specify only those fields you wish
to change, or else you may overwrite changes made by another user since
you last retrieved the task.

Returns the complete updated task record.*/
    pub fn update_task(
        &self,
        data: TaskRequest,
        task_gid: &str,
    ) -> FluentRequest<'_, request::UpdateTaskRequest> {
        FluentRequest {
            client: self,
            params: request::UpdateTaskRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Delete a task

A specific, existing task can be deleted by making a DELETE request on
the URL for that task. Deleted tasks go into the “trash” of the user
making the delete request. Tasks can be recovered from the trash within a
period of 30 days; afterward they are completely removed from the system.

Returns an empty data record.*/
    pub fn delete_task(
        &self,
        task_gid: &str,
    ) -> FluentRequest<'_, request::DeleteTaskRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteTaskRequest {
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Duplicate a task

Creates and returns a job that will asynchronously handle the duplication.*/
    pub fn duplicate_task(
        &self,
        data: TaskDuplicateRequest,
        task_gid: &str,
    ) -> FluentRequest<'_, request::DuplicateTaskRequest> {
        FluentRequest {
            client: self,
            params: request::DuplicateTaskRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Get tasks from a project

Returns the compact task records for all tasks within the given project, ordered by their priority within the project. Tasks can exist in more than one project at a time.*/
    pub fn get_tasks_for_project(
        &self,
        project_gid: &str,
    ) -> FluentRequest<'_, request::GetTasksForProjectRequest> {
        FluentRequest {
            client: self,
            params: request::GetTasksForProjectRequest {
                completed_since: None,
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                project_gid: project_gid.to_owned(),
            },
        }
    }
    /**Get tasks from a section

*Board view only*: Returns the compact section records for all tasks within the given section.*/
    pub fn get_tasks_for_section(
        &self,
        section_gid: &str,
    ) -> FluentRequest<'_, request::GetTasksForSectionRequest> {
        FluentRequest {
            client: self,
            params: request::GetTasksForSectionRequest {
                completed_since: None,
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                section_gid: section_gid.to_owned(),
            },
        }
    }
    /**Get tasks from a tag

Returns the compact task records for all tasks with the given tag. Tasks can have more than one tag at a time.*/
    pub fn get_tasks_for_tag(
        &self,
        tag_gid: &str,
    ) -> FluentRequest<'_, request::GetTasksForTagRequest> {
        FluentRequest {
            client: self,
            params: request::GetTasksForTagRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                tag_gid: tag_gid.to_owned(),
            },
        }
    }
    /**Get tasks from a user task list

Returns the compact list of tasks in a user’s My Tasks list.
*Note: Access control is enforced for this endpoint as with all Asana API endpoints, meaning a user’s private tasks will be filtered out if the API-authenticated user does not have access to them.*
*Note: Both complete and incomplete tasks are returned by default unless they are filtered out (for example, setting `completed_since=now` will return only incomplete tasks, which is the default view for “My Tasks” in Asana.)**/
    pub fn get_tasks_for_user_task_list(
        &self,
        user_task_list_gid: &str,
    ) -> FluentRequest<'_, request::GetTasksForUserTaskListRequest> {
        FluentRequest {
            client: self,
            params: request::GetTasksForUserTaskListRequest {
                completed_since: None,
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                user_task_list_gid: user_task_list_gid.to_owned(),
            },
        }
    }
    /**Get subtasks from a task

Returns a compact representation of all of the subtasks of a task.*/
    pub fn get_subtasks_for_task(
        &self,
        task_gid: &str,
    ) -> FluentRequest<'_, request::GetSubtasksForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::GetSubtasksForTaskRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Create a subtask

Creates a new subtask and adds it to the parent task. Returns the full record for the newly created subtask.*/
    pub fn create_subtask_for_task(
        &self,
        data: TaskRequest,
        task_gid: &str,
    ) -> FluentRequest<'_, request::CreateSubtaskForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::CreateSubtaskForTaskRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Set the parent of a task

parent, or no parent task at all. Returns an empty data block. When using `insert_before` and `insert_after`, at most one of those two options can be specified, and they must already be subtasks of the parent.*/
    pub fn set_parent_for_task(
        &self,
        data: TaskSetParentRequest,
        task_gid: &str,
    ) -> FluentRequest<'_, request::SetParentForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::SetParentForTaskRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Get dependencies from a task

Returns the compact representations of all of the dependencies of a task.*/
    pub fn get_dependencies_for_task(
        &self,
        task_gid: &str,
    ) -> FluentRequest<'_, request::GetDependenciesForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::GetDependenciesForTaskRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Set dependencies for a task

Marks a set of tasks as dependencies of this task, if they are not already dependencies. *A task can have at most 30 dependents and dependencies combined*.*/
    pub fn add_dependencies_for_task(
        &self,
        data: ModifyDependenciesRequest,
        task_gid: &str,
    ) -> FluentRequest<'_, request::AddDependenciesForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::AddDependenciesForTaskRequest {
                data,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Unlink dependencies from a task

Unlinks a set of dependencies from this task.*/
    pub fn remove_dependencies_for_task(
        &self,
        data: ModifyDependenciesRequest,
        task_gid: &str,
    ) -> FluentRequest<'_, request::RemoveDependenciesForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::RemoveDependenciesForTaskRequest {
                data,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Get dependents from a task

Returns the compact representations of all of the dependents of a task.*/
    pub fn get_dependents_for_task(
        &self,
        task_gid: &str,
    ) -> FluentRequest<'_, request::GetDependentsForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::GetDependentsForTaskRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Set dependents for a task

Marks a set of tasks as dependents of this task, if they are not already dependents. *A task can have at most 30 dependents and dependencies combined*.*/
    pub fn add_dependents_for_task(
        &self,
        data: ModifyDependentsRequest,
        task_gid: &str,
    ) -> FluentRequest<'_, request::AddDependentsForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::AddDependentsForTaskRequest {
                data,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Unlink dependents from a task

Unlinks a set of dependents from this task.*/
    pub fn remove_dependents_for_task(
        &self,
        data: ModifyDependentsRequest,
        task_gid: &str,
    ) -> FluentRequest<'_, request::RemoveDependentsForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::RemoveDependentsForTaskRequest {
                data,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Add a project to a task

Adds the task to the specified project, in the optional location
specified. If no location arguments are given, the task will be added to
the end of the project.

`addProject` can also be used to reorder a task within a project or
section that already contains it.

At most one of `insert_before`, `insert_after`, or `section` should be
specified. Inserting into a section in an non-order-dependent way can be
done by specifying section, otherwise, to insert within a section in a
particular place, specify `insert_before` or `insert_after` and a task
within the section to anchor the position of this task.

Returns an empty data block.*/
    pub fn add_project_for_task(
        &self,
        data: TaskAddProjectRequest,
        task_gid: &str,
    ) -> FluentRequest<'_, request::AddProjectForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::AddProjectForTaskRequest {
                data,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Remove a project from a task

Removes the task from the specified project. The task will still exist in
the system, but it will not be in the project anymore.

Returns an empty data block.*/
    pub fn remove_project_for_task(
        &self,
        data: TaskRemoveProjectRequest,
        task_gid: &str,
    ) -> FluentRequest<'_, request::RemoveProjectForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::RemoveProjectForTaskRequest {
                data,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Add a tag to a task

Adds a tag to a task. Returns an empty data block.*/
    pub fn add_tag_for_task(
        &self,
        data: TaskAddTagRequest,
        task_gid: &str,
    ) -> FluentRequest<'_, request::AddTagForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::AddTagForTaskRequest {
                data,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Remove a tag from a task

Removes a tag from a task. Returns an empty data block.*/
    pub fn remove_tag_for_task(
        &self,
        data: TaskRemoveTagRequest,
        task_gid: &str,
    ) -> FluentRequest<'_, request::RemoveTagForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::RemoveTagForTaskRequest {
                data,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Add followers to a task

Adds followers to a task. Returns an empty data block.
Each task can be associated with zero or more followers in the system.
Requests to add/remove followers, if successful, will return the complete updated task record, described above.*/
    pub fn add_followers_for_task(
        &self,
        data: TaskAddFollowersRequest,
        task_gid: &str,
    ) -> FluentRequest<'_, request::AddFollowersForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::AddFollowersForTaskRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Remove followers from a task

Removes each of the specified followers from the task if they are following. Returns the complete, updated record for the affected task.*/
    pub fn remove_follower_for_task(
        &self,
        data: TaskRemoveFollowersRequest,
        task_gid: &str,
    ) -> FluentRequest<'_, request::RemoveFollowerForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::RemoveFollowerForTaskRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Search tasks in a workspace

To mirror the functionality of the Asana web app's advanced search feature, the Asana API has a task search endpoint that allows you to build complex filters to find and retrieve the exact data you need.
#### Premium access
Like the Asana web product's advance search feature, this search endpoint will only be available to premium Asana users. A user is premium if any of the following is true:

- The workspace in which the search is being performed is a premium workspace - The user is a member of a premium team inside the workspace

Even if a user is only a member of a premium team inside a non-premium workspace, search will allow them to find data anywhere in the workspace, not just inside the premium team. Making a search request using credentials of a non-premium user will result in a `402 Payment Required` error.
#### Pagination
Search results are not stable; repeating the same query multiple times may return the data in a different order, even if the data do not change. Because of this, the traditional [pagination](https://developers.asana.com/docs/#pagination) available elsewhere in the Asana API is not available here. However, you can paginate manually by sorting the search results by their creation time and then modifying each subsequent query to exclude data you have already seen. Page sizes are limited to a maximum of 100 items, and can be specified by the `limit` query parameter.
#### Eventual consistency
Changes in Asana (regardless of whether they’re made though the web product or the API) are forwarded to our search infrastructure to be indexed. This process can take between 10 and 60 seconds to complete under normal operation, and longer during some production incidents. Making a change to a task that would alter its presence in a particular search query will not be reflected immediately. This is also true of the advanced search feature in the web product.
#### Rate limits
You may receive a `429 Too Many Requests` response if you hit any of our [rate limits](https://developers.asana.com/docs/#rate-limits).
#### Custom field parameters
| Parameter name | Custom field type | Accepted type |
|---|---|---|
| custom_fields.{gid}.is_set | All | Boolean |
| custom_fields.{gid}.value | Text | String |
| custom_fields.{gid}.value | Number | Number |
| custom_fields.{gid}.value | Enum | Enum option ID |
| custom_fields.{gid}.starts_with | Text only | String |
| custom_fields.{gid}.ends_with | Text only | String |
| custom_fields.{gid}.contains | Text only | String |
| custom_fields.{gid}.less_than | Number only | Number |
| custom_fields.{gid}.greater_than | Number only | Number |


For example, if the gid of the custom field is 12345, these query parameter to find tasks where it is set would be `custom_fields.12345.is_set=true`. To match an exact value for an enum custom field, use the gid of the desired enum option and not the name of the enum option: `custom_fields.12345.value=67890`.

**Not Supported**: searching for multiple exact matches of a custom field, searching for multi-enum custom field

*Note: If you specify `projects.any` and `sections.any`, you will receive tasks for the project **and** tasks for the section. If you're looking for only tasks in a section, omit the `projects.any` from the request.**/
    pub fn search_tasks_for_workspace(
        &self,
        workspace_gid: &str,
    ) -> FluentRequest<'_, request::SearchTasksForWorkspaceRequest> {
        FluentRequest {
            client: self,
            params: request::SearchTasksForWorkspaceRequest {
                assigned_by_any: None,
                assigned_by_not: None,
                assignee_any: None,
                assignee_not: None,
                commented_on_by_not: None,
                completed: None,
                completed_at_after: None,
                completed_at_before: None,
                completed_on: None,
                completed_on_after: None,
                completed_on_before: None,
                created_at_after: None,
                created_at_before: None,
                created_by_any: None,
                created_by_not: None,
                created_on: None,
                created_on_after: None,
                created_on_before: None,
                due_at_after: None,
                due_at_before: None,
                due_on: None,
                due_on_after: None,
                due_on_before: None,
                followers_not: None,
                has_attachment: None,
                is_blocked: None,
                is_blocking: None,
                is_subtask: None,
                liked_by_not: None,
                modified_at_after: None,
                modified_at_before: None,
                modified_on: None,
                modified_on_after: None,
                modified_on_before: None,
                opt_fields: None,
                opt_pretty: None,
                portfolios_any: None,
                projects_all: None,
                projects_any: None,
                projects_not: None,
                resource_subtype: None,
                sections_all: None,
                sections_any: None,
                sections_not: None,
                sort_ascending: None,
                sort_by: None,
                start_on: None,
                start_on_after: None,
                start_on_before: None,
                tags_all: None,
                tags_any: None,
                tags_not: None,
                teams_any: None,
                text: None,
                workspace_gid: workspace_gid.to_owned(),
            },
        }
    }
    /**Get a team membership

Returns the complete team membership record for a single team membership.*/
    pub fn get_team_membership(
        &self,
        team_membership_gid: &str,
    ) -> FluentRequest<'_, request::GetTeamMembershipRequest> {
        FluentRequest {
            client: self,
            params: request::GetTeamMembershipRequest {
                opt_fields: None,
                opt_pretty: None,
                team_membership_gid: team_membership_gid.to_owned(),
            },
        }
    }
    /**Get team memberships

Returns compact team membership records.*/
    pub fn get_team_memberships(
        &self,
    ) -> FluentRequest<'_, request::GetTeamMembershipsRequest> {
        FluentRequest {
            client: self,
            params: request::GetTeamMembershipsRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                team: None,
                user: None,
                workspace: None,
            },
        }
    }
    /**Get memberships from a team

Returns the compact team memberships for the team.*/
    pub fn get_team_memberships_for_team(
        &self,
        team_gid: &str,
    ) -> FluentRequest<'_, request::GetTeamMembershipsForTeamRequest> {
        FluentRequest {
            client: self,
            params: request::GetTeamMembershipsForTeamRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                team_gid: team_gid.to_owned(),
            },
        }
    }
    /**Get memberships from a user

Returns the compact team membership records for the user.*/
    pub fn get_team_memberships_for_user(
        &self,
        user_gid: &str,
        workspace: &str,
    ) -> FluentRequest<'_, request::GetTeamMembershipsForUserRequest> {
        FluentRequest {
            client: self,
            params: request::GetTeamMembershipsForUserRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                user_gid: user_gid.to_owned(),
                workspace: workspace.to_owned(),
            },
        }
    }
    /**Create a team

Creates a team within the current workspace.*/
    pub fn create_team(
        &self,
        data: TeamRequest,
    ) -> FluentRequest<'_, request::CreateTeamRequest> {
        FluentRequest {
            client: self,
            params: request::CreateTeamRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get a team

Returns the full record for a single team.*/
    pub fn get_team(
        &self,
        team_gid: &str,
    ) -> FluentRequest<'_, request::GetTeamRequest> {
        FluentRequest {
            client: self,
            params: request::GetTeamRequest {
                opt_fields: None,
                opt_pretty: None,
                team_gid: team_gid.to_owned(),
            },
        }
    }
    /**Update a team

Updates a team within the current workspace.*/
    pub fn update_team(
        &self,
        data: TeamRequest,
        team_gid: &str,
    ) -> FluentRequest<'_, request::UpdateTeamRequest> {
        FluentRequest {
            client: self,
            params: request::UpdateTeamRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                team_gid: team_gid.to_owned(),
            },
        }
    }
    /**Get teams in a workspace

Returns the compact records for all teams in the workspace visible to the authorized user.*/
    pub fn get_teams_for_workspace(
        &self,
        workspace_gid: &str,
    ) -> FluentRequest<'_, request::GetTeamsForWorkspaceRequest> {
        FluentRequest {
            client: self,
            params: request::GetTeamsForWorkspaceRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                workspace_gid: workspace_gid.to_owned(),
            },
        }
    }
    /**Get teams for a user

Returns the compact records for all teams to which the given user is assigned.*/
    pub fn get_teams_for_user(
        &self,
        organization: &str,
        user_gid: &str,
    ) -> FluentRequest<'_, request::GetTeamsForUserRequest> {
        FluentRequest {
            client: self,
            params: request::GetTeamsForUserRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                organization: organization.to_owned(),
                user_gid: user_gid.to_owned(),
            },
        }
    }
    /**Add a user to a team

The user making this call must be a member of the team in order to add others. The user being added must exist in the same organization as the team.

Returns the complete team membership record for the newly added user.*/
    pub fn add_user_for_team(
        &self,
        data: TeamAddUserRequest,
        team_gid: &str,
    ) -> FluentRequest<'_, request::AddUserForTeamRequest> {
        FluentRequest {
            client: self,
            params: request::AddUserForTeamRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                team_gid: team_gid.to_owned(),
            },
        }
    }
    /**Remove a user from a team

The user making this call must be a member of the team in order to remove themselves or others.*/
    pub fn remove_user_for_team(
        &self,
        data: TeamRemoveUserRequest,
        team_gid: &str,
    ) -> FluentRequest<'_, request::RemoveUserForTeamRequest> {
        FluentRequest {
            client: self,
            params: request::RemoveUserForTeamRequest {
                data,
                opt_pretty: None,
                team_gid: team_gid.to_owned(),
            },
        }
    }
    /**Get a time period

Returns the full record for a single time period.*/
    pub fn get_time_period(
        &self,
        time_period_gid: &str,
    ) -> FluentRequest<'_, request::GetTimePeriodRequest> {
        FluentRequest {
            client: self,
            params: request::GetTimePeriodRequest {
                opt_fields: None,
                opt_pretty: None,
                time_period_gid: time_period_gid.to_owned(),
            },
        }
    }
    /**Get time periods

Returns compact time period records.*/
    pub fn get_time_periods(
        &self,
        workspace: &str,
    ) -> FluentRequest<'_, request::GetTimePeriodsRequest> {
        FluentRequest {
            client: self,
            params: request::GetTimePeriodsRequest {
                end_on: None,
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                start_on: None,
                workspace: workspace.to_owned(),
            },
        }
    }
    /**Get time tracking entries for a task

Returns time tracking entries for a given task.*/
    pub fn get_time_tracking_entries_for_task(
        &self,
        task_gid: &str,
    ) -> FluentRequest<'_, request::GetTimeTrackingEntriesForTaskRequest> {
        FluentRequest {
            client: self,
            params: request::GetTimeTrackingEntriesForTaskRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Create a time tracking entry

Creates a time tracking entry on a given task.

Returns the record of the newly created time tracking entry.*/
    pub fn create_time_tracking_entry(
        &self,
        data: CreateTimeTrackingEntryRequestBody,
        task_gid: &str,
    ) -> FluentRequest<'_, request::CreateTimeTrackingEntryRequest> {
        FluentRequest {
            client: self,
            params: request::CreateTimeTrackingEntryRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                task_gid: task_gid.to_owned(),
            },
        }
    }
    /**Get a time tracking entry

Returns the complete time tracking entry record for a single time tracking entry.*/
    pub fn get_time_tracking_entry(
        &self,
        time_tracking_entry_gid: &str,
    ) -> FluentRequest<'_, request::GetTimeTrackingEntryRequest> {
        FluentRequest {
            client: self,
            params: request::GetTimeTrackingEntryRequest {
                opt_fields: None,
                opt_pretty: None,
                time_tracking_entry_gid: time_tracking_entry_gid.to_owned(),
            },
        }
    }
    /**Update a time tracking entry

A specific, existing time tracking entry can be updated by making a `PUT` request on
the URL for that time tracking entry. Only the fields provided in the `data` block
will be updated; any unspecified fields will remain unchanged.

When using this method, it is best to specify only those fields you wish
to change, or else you may overwrite changes made by another user since
you last retrieved the task.

Returns the complete updated time tracking entry record.*/
    pub fn update_time_tracking_entry(
        &self,
        data: UpdateTimeTrackingEntryRequestBody,
        time_tracking_entry_gid: &str,
    ) -> FluentRequest<'_, request::UpdateTimeTrackingEntryRequest> {
        FluentRequest {
            client: self,
            params: request::UpdateTimeTrackingEntryRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                time_tracking_entry_gid: time_tracking_entry_gid.to_owned(),
            },
        }
    }
    /**Delete a time tracking entry

A specific, existing time tracking entry can be deleted by making a `DELETE` request on
the URL for that time tracking entry.

Returns an empty data record.*/
    pub fn delete_time_tracking_entry(
        &self,
        time_tracking_entry_gid: &str,
    ) -> FluentRequest<'_, request::DeleteTimeTrackingEntryRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteTimeTrackingEntryRequest {
                opt_pretty: None,
                time_tracking_entry_gid: time_tracking_entry_gid.to_owned(),
            },
        }
    }
    /**Get objects via typeahead

Retrieves objects in the workspace based via an auto-completion/typeahead
search algorithm. This feature is meant to provide results quickly, so do
not rely on this API to provide extremely accurate search results. The
result set is limited to a single page of results with a maximum size, so
you won’t be able to fetch large numbers of results.

The typeahead search API provides search for objects from a single
workspace. This endpoint should be used to query for objects when
creating an auto-completion/typeahead search feature. This API is meant
to provide results quickly and should not be relied upon for accurate or
exhaustive search results. The results sets are limited in size and
cannot be paginated.

Queries return a compact representation of each object which is typically
the gid and name fields. Interested in a specific set of fields or all of
the fields?! Of course you are. Use field selectors to manipulate what
data is included in a response.

Resources with type `user` are returned in order of most contacted to
least contacted. This is determined by task assignments, adding the user
to projects, and adding the user as a follower to tasks, messages,
etc.

Resources with type `project` are returned in order of recency. This is
determined when the user visits the project, is added to the project, and
completes tasks in the project.

Resources with type `task` are returned with priority placed on tasks
the user is following, but no guarantee on the order of those tasks.

Resources with type `project_template` are returned with priority
placed on favorited project templates.

Leaving the `query` string empty or omitted will give you results, still
following the resource ordering above. This could be used to list users or
projects that are relevant for the requesting user's api token.*/
    pub fn typeahead_for_workspace(
        &self,
        resource_type: &str,
        workspace_gid: &str,
    ) -> FluentRequest<'_, request::TypeaheadForWorkspaceRequest> {
        FluentRequest {
            client: self,
            params: request::TypeaheadForWorkspaceRequest {
                count: None,
                opt_fields: None,
                opt_pretty: None,
                query: None,
                resource_type: resource_type.to_owned(),
                type_: None,
                workspace_gid: workspace_gid.to_owned(),
            },
        }
    }
    /**Get a user task list

Returns the full record for a user task list.*/
    pub fn get_user_task_list(
        &self,
        user_task_list_gid: &str,
    ) -> FluentRequest<'_, request::GetUserTaskListRequest> {
        FluentRequest {
            client: self,
            params: request::GetUserTaskListRequest {
                opt_fields: None,
                opt_pretty: None,
                user_task_list_gid: user_task_list_gid.to_owned(),
            },
        }
    }
    /**Get a user's task list

Returns the full record for a user's task list.*/
    pub fn get_user_task_list_for_user(
        &self,
        user_gid: &str,
        workspace: &str,
    ) -> FluentRequest<'_, request::GetUserTaskListForUserRequest> {
        FluentRequest {
            client: self,
            params: request::GetUserTaskListForUserRequest {
                opt_fields: None,
                opt_pretty: None,
                user_gid: user_gid.to_owned(),
                workspace: workspace.to_owned(),
            },
        }
    }
    /**Get multiple users

Returns the user records for all users in all workspaces and organizations accessible to the authenticated user. Accepts an optional workspace ID parameter.
Results are sorted by user ID.*/
    pub fn get_users(&self) -> FluentRequest<'_, request::GetUsersRequest> {
        FluentRequest {
            client: self,
            params: request::GetUsersRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                team: None,
                workspace: None,
            },
        }
    }
    /**Get a user

Returns the full user record for the single user with the provided ID.*/
    pub fn get_user(
        &self,
        user_gid: &str,
    ) -> FluentRequest<'_, request::GetUserRequest> {
        FluentRequest {
            client: self,
            params: request::GetUserRequest {
                opt_fields: None,
                opt_pretty: None,
                user_gid: user_gid.to_owned(),
            },
        }
    }
    /**Get a user's favorites

Returns all of a user's favorites in the given workspace, of the given type.
Results are given in order (The same order as Asana's sidebar).*/
    pub fn get_favorites_for_user(
        &self,
        resource_type: &str,
        user_gid: &str,
        workspace: &str,
    ) -> FluentRequest<'_, request::GetFavoritesForUserRequest> {
        FluentRequest {
            client: self,
            params: request::GetFavoritesForUserRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                resource_type: resource_type.to_owned(),
                user_gid: user_gid.to_owned(),
                workspace: workspace.to_owned(),
            },
        }
    }
    /**Get users in a team

Returns the compact records for all users that are members of the team.
Results are sorted alphabetically and limited to 2000. For more results use the `/users` endpoint.*/
    pub fn get_users_for_team(
        &self,
        team_gid: &str,
    ) -> FluentRequest<'_, request::GetUsersForTeamRequest> {
        FluentRequest {
            client: self,
            params: request::GetUsersForTeamRequest {
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                team_gid: team_gid.to_owned(),
            },
        }
    }
    /**Get users in a workspace or organization

Returns the compact records for all users in the specified workspace or organization.
Results are sorted alphabetically and limited to 2000. For more results use the `/users` endpoint.*/
    pub fn get_users_for_workspace(
        &self,
        workspace_gid: &str,
    ) -> FluentRequest<'_, request::GetUsersForWorkspaceRequest> {
        FluentRequest {
            client: self,
            params: request::GetUsersForWorkspaceRequest {
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                workspace_gid: workspace_gid.to_owned(),
            },
        }
    }
    /**Get multiple webhooks

Get the compact representation of all webhooks your app has registered for the authenticated user in the given workspace.*/
    pub fn get_webhooks(
        &self,
        workspace: &str,
    ) -> FluentRequest<'_, request::GetWebhooksRequest> {
        FluentRequest {
            client: self,
            params: request::GetWebhooksRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                resource: None,
                workspace: workspace.to_owned(),
            },
        }
    }
    /**Establish a webhook

Establishing a webhook is a two-part process. First, a simple HTTP POST
request initiates the creation similar to creating any other resource.

Next, in the middle of this request comes the confirmation handshake.
When a webhook is created, we will send a test POST to the target with an
`X-Hook-Secret` header. The target must respond with a `200 OK` or `204
No Content` and a matching `X-Hook-Secret` header to confirm that this
webhook subscription is indeed expected. We strongly recommend storing
this secret to be used to verify future webhook event signatures.

The POST request to create the webhook will then return with the status
of the request. If you do not acknowledge the webhook’s confirmation
handshake it will fail to setup, and you will receive an error in
response to your attempt to create it. This means you need to be able to
receive and complete the webhook *while* the POST request is in-flight
(in other words, have a server that can handle requests asynchronously).

Invalid hostnames like localhost will recieve a 403 Forbidden status code.

```
# Request
curl -H "Authorization: Bearer <personal_access_token>" \
-X POST https://app.asana.com/api/1.0/webhooks \
-d "resource=8675309" \
-d "target=https://example.com/receive-webhook/7654"
```

```
# Handshake sent to https://example.com/
POST /receive-webhook/7654
X-Hook-Secret: b537207f20cbfa02357cf448134da559e8bd39d61597dcd5631b8012eae53e81
```

```
# Handshake response sent by example.com
HTTP/1.1 200
X-Hook-Secret: b537207f20cbfa02357cf448134da559e8bd39d61597dcd5631b8012eae53e81
```

```
# Response
HTTP/1.1 201
{
  "data": {
    "gid": "43214",
    "resource": {
      "gid": "8675309",
      "name": "Bugs"
    },
    "target": "https://example.com/receive-webhook/7654",
    "active": false,
    "last_success_at": null,
    "last_failure_at": null,
    "last_failure_content": null
  }
}
```*/
    pub fn create_webhook(
        &self,
        data: WebhookRequest,
    ) -> FluentRequest<'_, request::CreateWebhookRequest> {
        FluentRequest {
            client: self,
            params: request::CreateWebhookRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get a webhook

Returns the full record for the given webhook.*/
    pub fn get_webhook(
        &self,
        webhook_gid: &str,
    ) -> FluentRequest<'_, request::GetWebhookRequest> {
        FluentRequest {
            client: self,
            params: request::GetWebhookRequest {
                opt_fields: None,
                opt_pretty: None,
                webhook_gid: webhook_gid.to_owned(),
            },
        }
    }
    /**Update a webhook

An existing webhook's filters can be updated by making a PUT request on the URL for that webhook. Note that the webhook's previous `filters` array will be completely overwritten by the `filters` sent in the PUT request.*/
    pub fn update_webhook(
        &self,
        data: WebhookUpdateRequest,
        webhook_gid: &str,
    ) -> FluentRequest<'_, request::UpdateWebhookRequest> {
        FluentRequest {
            client: self,
            params: request::UpdateWebhookRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                webhook_gid: webhook_gid.to_owned(),
            },
        }
    }
    /**Delete a webhook

This method *permanently* removes a webhook. Note that it may be possible to receive a request that was already in flight after deleting the webhook, but no further requests will be issued.*/
    pub fn delete_webhook(
        &self,
        webhook_gid: &str,
    ) -> FluentRequest<'_, request::DeleteWebhookRequest> {
        FluentRequest {
            client: self,
            params: request::DeleteWebhookRequest {
                opt_pretty: None,
                webhook_gid: webhook_gid.to_owned(),
            },
        }
    }
    /**Get a workspace membership

Returns the complete workspace record for a single workspace membership.*/
    pub fn get_workspace_membership(
        &self,
        workspace_membership_gid: &str,
    ) -> FluentRequest<'_, request::GetWorkspaceMembershipRequest> {
        FluentRequest {
            client: self,
            params: request::GetWorkspaceMembershipRequest {
                opt_fields: None,
                opt_pretty: None,
                workspace_membership_gid: workspace_membership_gid.to_owned(),
            },
        }
    }
    /**Get workspace memberships for a user

Returns the compact workspace membership records for the user.*/
    pub fn get_workspace_memberships_for_user(
        &self,
        user_gid: &str,
    ) -> FluentRequest<'_, request::GetWorkspaceMembershipsForUserRequest> {
        FluentRequest {
            client: self,
            params: request::GetWorkspaceMembershipsForUserRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                user_gid: user_gid.to_owned(),
            },
        }
    }
    /**Get the workspace memberships for a workspace

Returns the compact workspace membership records for the workspace.*/
    pub fn get_workspace_memberships_for_workspace(
        &self,
        workspace_gid: &str,
    ) -> FluentRequest<'_, request::GetWorkspaceMembershipsForWorkspaceRequest> {
        FluentRequest {
            client: self,
            params: request::GetWorkspaceMembershipsForWorkspaceRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
                user: None,
                workspace_gid: workspace_gid.to_owned(),
            },
        }
    }
    /**Get multiple workspaces

Returns the compact records for all workspaces visible to the authorized user.*/
    pub fn get_workspaces(&self) -> FluentRequest<'_, request::GetWorkspacesRequest> {
        FluentRequest {
            client: self,
            params: request::GetWorkspacesRequest {
                limit: None,
                offset: None,
                opt_fields: None,
                opt_pretty: None,
            },
        }
    }
    /**Get a workspace

Returns the full workspace record for a single workspace.*/
    pub fn get_workspace(
        &self,
        workspace_gid: &str,
    ) -> FluentRequest<'_, request::GetWorkspaceRequest> {
        FluentRequest {
            client: self,
            params: request::GetWorkspaceRequest {
                opt_fields: None,
                opt_pretty: None,
                workspace_gid: workspace_gid.to_owned(),
            },
        }
    }
    /**Update a workspace

A specific, existing workspace can be updated by making a PUT request on the URL for that workspace. Only the fields provided in the data block will be updated; any unspecified fields will remain unchanged.
Currently the only field that can be modified for a workspace is its name.
Returns the complete, updated workspace record.*/
    pub fn update_workspace(
        &self,
        data: WorkspaceRequest,
        workspace_gid: &str,
    ) -> FluentRequest<'_, request::UpdateWorkspaceRequest> {
        FluentRequest {
            client: self,
            params: request::UpdateWorkspaceRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                workspace_gid: workspace_gid.to_owned(),
            },
        }
    }
    /**Add a user to a workspace or organization

Add a user to a workspace or organization.
The user can be referenced by their globally unique user ID or their email address. Returns the full user record for the invited user.*/
    pub fn add_user_for_workspace(
        &self,
        data: WorkspaceAddUserRequest,
        workspace_gid: &str,
    ) -> FluentRequest<'_, request::AddUserForWorkspaceRequest> {
        FluentRequest {
            client: self,
            params: request::AddUserForWorkspaceRequest {
                data,
                opt_fields: None,
                opt_pretty: None,
                workspace_gid: workspace_gid.to_owned(),
            },
        }
    }
    /**Remove a user from a workspace or organization

Remove a user from a workspace or organization.
The user making this call must be an admin in the workspace. The user can be referenced by their globally unique user ID or their email address.
Returns an empty data record.*/
    pub fn remove_user_for_workspace(
        &self,
        data: WorkspaceRemoveUserRequest,
        workspace_gid: &str,
    ) -> FluentRequest<'_, request::RemoveUserForWorkspaceRequest> {
        FluentRequest {
            client: self,
            params: request::RemoveUserForWorkspaceRequest {
                data,
                opt_pretty: None,
                workspace_gid: workspace_gid.to_owned(),
            },
        }
    }
}
pub enum AsanaAuthentication {
    PersonalAccessToken { personal_access_token: String },
}
impl AsanaAuthentication {
    pub fn from_env() -> Self {
        Self::PersonalAccessToken {
            personal_access_token: std::env::var("ASANA_PERSONAL_ACCESS_TOKEN")
                .expect("Environment variable PERSONAL_ACCESS_TOKEN is not set."),
        }
    }
}