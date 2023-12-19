pub mod get_attachment;
pub mod delete_attachment;
pub mod get_attachments_for_object;
pub mod create_attachment_for_object;
pub mod get_audit_log_events;
pub mod create_batch_request;
pub mod get_custom_field_settings_for_project;
pub mod get_custom_field_settings_for_portfolio;
pub mod create_custom_field;
pub mod get_custom_field;
pub mod update_custom_field;
pub mod delete_custom_field;
pub mod get_custom_fields_for_workspace;
pub mod create_enum_option_for_custom_field;
pub mod insert_enum_option_for_custom_field;
pub mod update_enum_option;
pub mod get_events;
pub mod get_goal_relationship;
pub mod update_goal_relationship;
pub mod get_goal_relationships;
pub mod add_supporting_relationship;
pub mod remove_supporting_relationship;
pub mod get_goal;
pub mod update_goal;
pub mod delete_goal;
pub mod get_goals;
pub mod create_goal;
pub mod create_goal_metric;
pub mod update_goal_metric;
pub mod add_followers;
pub mod remove_followers;
pub mod get_parent_goals_for_goal;
pub mod get_job;
pub mod get_memberships;
pub mod create_membership;
pub mod get_membership;
pub mod delete_membership;
pub mod create_organization_export;
pub mod get_organization_export;
pub mod get_portfolio_memberships;
pub mod get_portfolio_membership;
pub mod get_portfolio_memberships_for_portfolio;
pub mod get_portfolios;
pub mod create_portfolio;
pub mod get_portfolio;
pub mod update_portfolio;
pub mod delete_portfolio;
pub mod get_items_for_portfolio;
pub mod add_item_for_portfolio;
pub mod remove_item_for_portfolio;
pub mod add_custom_field_setting_for_portfolio;
pub mod remove_custom_field_setting_for_portfolio;
pub mod add_members_for_portfolio;
pub mod remove_members_for_portfolio;
pub mod get_project_brief;
pub mod update_project_brief;
pub mod delete_project_brief;
pub mod create_project_brief;
pub mod get_project_membership;
pub mod get_project_memberships_for_project;
pub mod get_project_status;
pub mod delete_project_status;
pub mod get_project_statuses_for_project;
pub mod create_project_status_for_project;
pub mod get_project_template;
pub mod delete_project_template;
pub mod get_project_templates;
pub mod get_project_templates_for_team;
pub mod instantiate_project;
pub mod get_projects;
pub mod create_project;
pub mod get_project;
pub mod update_project;
pub mod delete_project;
pub mod duplicate_project;
pub mod get_projects_for_task;
pub mod get_projects_for_team;
pub mod create_project_for_team;
pub mod get_projects_for_workspace;
pub mod create_project_for_workspace;
pub mod add_custom_field_setting_for_project;
pub mod remove_custom_field_setting_for_project;
pub mod get_task_counts_for_project;
pub mod add_members_for_project;
pub mod remove_members_for_project;
pub mod add_followers_for_project;
pub mod remove_followers_for_project;
pub mod project_save_as_template;
pub mod trigger_rule;
pub mod get_section;
pub mod update_section;
pub mod delete_section;
pub mod get_sections_for_project;
pub mod create_section_for_project;
pub mod add_task_for_section;
pub mod insert_section_for_project;
pub mod get_status;
pub mod delete_status;
pub mod get_statuses_for_object;
pub mod create_status_for_object;
pub mod get_story;
pub mod update_story;
pub mod delete_story;
pub mod get_stories_for_task;
pub mod create_story_for_task;
pub mod get_tags;
pub mod create_tag;
pub mod get_tag;
pub mod update_tag;
pub mod delete_tag;
pub mod get_tags_for_task;
pub mod get_tags_for_workspace;
pub mod create_tag_for_workspace;
pub mod get_task_templates;
pub mod get_task_template;
pub mod instantiate_task;
pub mod get_tasks;
pub mod create_task;
pub mod get_task;
pub mod update_task;
pub mod delete_task;
pub mod duplicate_task;
pub mod get_tasks_for_project;
pub mod get_tasks_for_section;
pub mod get_tasks_for_tag;
pub mod get_tasks_for_user_task_list;
pub mod get_subtasks_for_task;
pub mod create_subtask_for_task;
pub mod set_parent_for_task;
pub mod get_dependencies_for_task;
pub mod add_dependencies_for_task;
pub mod remove_dependencies_for_task;
pub mod get_dependents_for_task;
pub mod add_dependents_for_task;
pub mod remove_dependents_for_task;
pub mod add_project_for_task;
pub mod remove_project_for_task;
pub mod add_tag_for_task;
pub mod remove_tag_for_task;
pub mod add_followers_for_task;
pub mod remove_follower_for_task;
pub mod search_tasks_for_workspace;
pub mod get_team_membership;
pub mod get_team_memberships;
pub mod get_team_memberships_for_team;
pub mod get_team_memberships_for_user;
pub mod create_team;
pub mod get_team;
pub mod update_team;
pub mod get_teams_for_workspace;
pub mod get_teams_for_user;
pub mod add_user_for_team;
pub mod remove_user_for_team;
pub mod get_time_period;
pub mod get_time_periods;
pub mod get_time_tracking_entries_for_task;
pub mod create_time_tracking_entry;
pub mod get_time_tracking_entry;
pub mod update_time_tracking_entry;
pub mod delete_time_tracking_entry;
pub mod typeahead_for_workspace;
pub mod get_user_task_list;
pub mod get_user_task_list_for_user;
pub mod get_users;
pub mod get_user;
pub mod get_favorites_for_user;
pub mod get_users_for_team;
pub mod get_users_for_workspace;
pub mod get_webhooks;
pub mod create_webhook;
pub mod get_webhook;
pub mod update_webhook;
pub mod delete_webhook;
pub mod get_workspace_membership;
pub mod get_workspace_memberships_for_user;
pub mod get_workspace_memberships_for_workspace;
pub mod get_workspaces;
pub mod get_workspace;
pub mod update_workspace;
pub mod add_user_for_workspace;
pub mod remove_user_for_workspace;
pub use get_attachment::GetAttachmentRequest;
pub use delete_attachment::DeleteAttachmentRequest;
pub use get_attachments_for_object::GetAttachmentsForObjectRequest;
pub use create_attachment_for_object::CreateAttachmentForObjectRequest;
pub use get_audit_log_events::GetAuditLogEventsRequest;
pub use create_batch_request::CreateBatchRequestRequest;
pub use get_custom_field_settings_for_project::GetCustomFieldSettingsForProjectRequest;
pub use get_custom_field_settings_for_portfolio::GetCustomFieldSettingsForPortfolioRequest;
pub use create_custom_field::CreateCustomFieldRequest;
pub use get_custom_field::GetCustomFieldRequest;
pub use update_custom_field::UpdateCustomFieldRequest;
pub use delete_custom_field::DeleteCustomFieldRequest;
pub use get_custom_fields_for_workspace::GetCustomFieldsForWorkspaceRequest;
pub use create_enum_option_for_custom_field::CreateEnumOptionForCustomFieldRequest;
pub use insert_enum_option_for_custom_field::InsertEnumOptionForCustomFieldRequest;
pub use update_enum_option::UpdateEnumOptionRequest;
pub use get_events::GetEventsRequest;
pub use get_goal_relationship::GetGoalRelationshipRequest;
pub use update_goal_relationship::UpdateGoalRelationshipRequest;
pub use get_goal_relationships::GetGoalRelationshipsRequest;
pub use add_supporting_relationship::AddSupportingRelationshipRequest;
pub use remove_supporting_relationship::RemoveSupportingRelationshipRequest;
pub use get_goal::GetGoalRequest;
pub use update_goal::UpdateGoalRequest;
pub use delete_goal::DeleteGoalRequest;
pub use get_goals::GetGoalsRequest;
pub use create_goal::CreateGoalRequest;
pub use create_goal_metric::CreateGoalMetricRequest;
pub use update_goal_metric::UpdateGoalMetricRequest;
pub use add_followers::AddFollowersRequest;
pub use remove_followers::RemoveFollowersRequest;
pub use get_parent_goals_for_goal::GetParentGoalsForGoalRequest;
pub use get_job::GetJobRequest;
pub use get_memberships::GetMembershipsRequest;
pub use create_membership::CreateMembershipRequest;
pub use get_membership::GetMembershipRequest;
pub use delete_membership::DeleteMembershipRequest;
pub use create_organization_export::CreateOrganizationExportRequest;
pub use get_organization_export::GetOrganizationExportRequest;
pub use get_portfolio_memberships::GetPortfolioMembershipsRequest;
pub use get_portfolio_membership::GetPortfolioMembershipRequest;
pub use get_portfolio_memberships_for_portfolio::GetPortfolioMembershipsForPortfolioRequest;
pub use get_portfolios::GetPortfoliosRequest;
pub use create_portfolio::CreatePortfolioRequest;
pub use get_portfolio::GetPortfolioRequest;
pub use update_portfolio::UpdatePortfolioRequest;
pub use delete_portfolio::DeletePortfolioRequest;
pub use get_items_for_portfolio::GetItemsForPortfolioRequest;
pub use add_item_for_portfolio::AddItemForPortfolioRequest;
pub use remove_item_for_portfolio::RemoveItemForPortfolioRequest;
pub use add_custom_field_setting_for_portfolio::AddCustomFieldSettingForPortfolioRequest;
pub use remove_custom_field_setting_for_portfolio::RemoveCustomFieldSettingForPortfolioRequest;
pub use add_members_for_portfolio::AddMembersForPortfolioRequest;
pub use remove_members_for_portfolio::RemoveMembersForPortfolioRequest;
pub use get_project_brief::GetProjectBriefRequest;
pub use update_project_brief::UpdateProjectBriefRequest;
pub use delete_project_brief::DeleteProjectBriefRequest;
pub use create_project_brief::CreateProjectBriefRequest;
pub use get_project_membership::GetProjectMembershipRequest;
pub use get_project_memberships_for_project::GetProjectMembershipsForProjectRequest;
pub use get_project_status::GetProjectStatusRequest;
pub use delete_project_status::DeleteProjectStatusRequest;
pub use get_project_statuses_for_project::GetProjectStatusesForProjectRequest;
pub use create_project_status_for_project::CreateProjectStatusForProjectRequest;
pub use get_project_template::GetProjectTemplateRequest;
pub use delete_project_template::DeleteProjectTemplateRequest;
pub use get_project_templates::GetProjectTemplatesRequest;
pub use get_project_templates_for_team::GetProjectTemplatesForTeamRequest;
pub use instantiate_project::InstantiateProjectRequest;
pub use get_projects::GetProjectsRequest;
pub use create_project::CreateProjectRequest;
pub use get_project::GetProjectRequest;
pub use update_project::UpdateProjectRequest;
pub use delete_project::DeleteProjectRequest;
pub use duplicate_project::DuplicateProjectRequest;
pub use get_projects_for_task::GetProjectsForTaskRequest;
pub use get_projects_for_team::GetProjectsForTeamRequest;
pub use create_project_for_team::CreateProjectForTeamRequest;
pub use get_projects_for_workspace::GetProjectsForWorkspaceRequest;
pub use create_project_for_workspace::CreateProjectForWorkspaceRequest;
pub use add_custom_field_setting_for_project::AddCustomFieldSettingForProjectRequest;
pub use remove_custom_field_setting_for_project::RemoveCustomFieldSettingForProjectRequest;
pub use get_task_counts_for_project::GetTaskCountsForProjectRequest;
pub use add_members_for_project::AddMembersForProjectRequest;
pub use remove_members_for_project::RemoveMembersForProjectRequest;
pub use add_followers_for_project::AddFollowersForProjectRequest;
pub use remove_followers_for_project::RemoveFollowersForProjectRequest;
pub use project_save_as_template::ProjectSaveAsTemplateRequest;
pub use trigger_rule::TriggerRuleRequest;
pub use get_section::GetSectionRequest;
pub use update_section::UpdateSectionRequest;
pub use delete_section::DeleteSectionRequest;
pub use get_sections_for_project::GetSectionsForProjectRequest;
pub use create_section_for_project::CreateSectionForProjectRequest;
pub use add_task_for_section::AddTaskForSectionRequest;
pub use insert_section_for_project::InsertSectionForProjectRequest;
pub use get_status::GetStatusRequest;
pub use delete_status::DeleteStatusRequest;
pub use get_statuses_for_object::GetStatusesForObjectRequest;
pub use create_status_for_object::CreateStatusForObjectRequest;
pub use get_story::GetStoryRequest;
pub use update_story::UpdateStoryRequest;
pub use delete_story::DeleteStoryRequest;
pub use get_stories_for_task::GetStoriesForTaskRequest;
pub use create_story_for_task::CreateStoryForTaskRequest;
pub use get_tags::GetTagsRequest;
pub use create_tag::CreateTagRequest;
pub use get_tag::GetTagRequest;
pub use update_tag::UpdateTagRequest;
pub use delete_tag::DeleteTagRequest;
pub use get_tags_for_task::GetTagsForTaskRequest;
pub use get_tags_for_workspace::GetTagsForWorkspaceRequest;
pub use create_tag_for_workspace::CreateTagForWorkspaceRequest;
pub use get_task_templates::GetTaskTemplatesRequest;
pub use get_task_template::GetTaskTemplateRequest;
pub use instantiate_task::InstantiateTaskRequest;
pub use get_tasks::GetTasksRequest;
pub use create_task::CreateTaskRequest;
pub use get_task::GetTaskRequest;
pub use update_task::UpdateTaskRequest;
pub use delete_task::DeleteTaskRequest;
pub use duplicate_task::DuplicateTaskRequest;
pub use get_tasks_for_project::GetTasksForProjectRequest;
pub use get_tasks_for_section::GetTasksForSectionRequest;
pub use get_tasks_for_tag::GetTasksForTagRequest;
pub use get_tasks_for_user_task_list::GetTasksForUserTaskListRequest;
pub use get_subtasks_for_task::GetSubtasksForTaskRequest;
pub use create_subtask_for_task::CreateSubtaskForTaskRequest;
pub use set_parent_for_task::SetParentForTaskRequest;
pub use get_dependencies_for_task::GetDependenciesForTaskRequest;
pub use add_dependencies_for_task::AddDependenciesForTaskRequest;
pub use remove_dependencies_for_task::RemoveDependenciesForTaskRequest;
pub use get_dependents_for_task::GetDependentsForTaskRequest;
pub use add_dependents_for_task::AddDependentsForTaskRequest;
pub use remove_dependents_for_task::RemoveDependentsForTaskRequest;
pub use add_project_for_task::AddProjectForTaskRequest;
pub use remove_project_for_task::RemoveProjectForTaskRequest;
pub use add_tag_for_task::AddTagForTaskRequest;
pub use remove_tag_for_task::RemoveTagForTaskRequest;
pub use add_followers_for_task::AddFollowersForTaskRequest;
pub use remove_follower_for_task::RemoveFollowerForTaskRequest;
pub use search_tasks_for_workspace::SearchTasksForWorkspaceRequest;
pub use get_team_membership::GetTeamMembershipRequest;
pub use get_team_memberships::GetTeamMembershipsRequest;
pub use get_team_memberships_for_team::GetTeamMembershipsForTeamRequest;
pub use get_team_memberships_for_user::GetTeamMembershipsForUserRequest;
pub use create_team::CreateTeamRequest;
pub use get_team::GetTeamRequest;
pub use update_team::UpdateTeamRequest;
pub use get_teams_for_workspace::GetTeamsForWorkspaceRequest;
pub use get_teams_for_user::GetTeamsForUserRequest;
pub use add_user_for_team::AddUserForTeamRequest;
pub use remove_user_for_team::RemoveUserForTeamRequest;
pub use get_time_period::GetTimePeriodRequest;
pub use get_time_periods::GetTimePeriodsRequest;
pub use get_time_tracking_entries_for_task::GetTimeTrackingEntriesForTaskRequest;
pub use create_time_tracking_entry::CreateTimeTrackingEntryRequest;
pub use get_time_tracking_entry::GetTimeTrackingEntryRequest;
pub use update_time_tracking_entry::UpdateTimeTrackingEntryRequest;
pub use delete_time_tracking_entry::DeleteTimeTrackingEntryRequest;
pub use typeahead_for_workspace::TypeaheadForWorkspaceRequest;
pub use get_user_task_list::GetUserTaskListRequest;
pub use get_user_task_list_for_user::GetUserTaskListForUserRequest;
pub use get_users::GetUsersRequest;
pub use get_user::GetUserRequest;
pub use get_favorites_for_user::GetFavoritesForUserRequest;
pub use get_users_for_team::GetUsersForTeamRequest;
pub use get_users_for_workspace::GetUsersForWorkspaceRequest;
pub use get_webhooks::GetWebhooksRequest;
pub use create_webhook::CreateWebhookRequest;
pub use get_webhook::GetWebhookRequest;
pub use update_webhook::UpdateWebhookRequest;
pub use delete_webhook::DeleteWebhookRequest;
pub use get_workspace_membership::GetWorkspaceMembershipRequest;
pub use get_workspace_memberships_for_user::GetWorkspaceMembershipsForUserRequest;
pub use get_workspace_memberships_for_workspace::GetWorkspaceMembershipsForWorkspaceRequest;
pub use get_workspaces::GetWorkspacesRequest;
pub use get_workspace::GetWorkspaceRequest;
pub use update_workspace::UpdateWorkspaceRequest;
pub use add_user_for_workspace::AddUserForWorkspaceRequest;
pub use remove_user_for_workspace::RemoveUserForWorkspaceRequest;
