
use serde::{Serialize, Deserialize};
use super::{AsanaResource, Like, TaskCompact};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct TaskBase {
    #[serde(flatten)]
    pub task_compact: TaskCompact,
    ///This value represents the sum of all the Time Tracking entries in the Actual Time field on a given Task. It is represented as a nullable long value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_time_minutes: Option<f64>,
    ///*Conditional* Reflects the approval status of this task. This field is kept in sync with `completed`, meaning `pending` translates to false while `approved`, `rejected`, and `changes_requested` translate to true. If you set completed to true, this field will be set to `approved`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_status: Option<String>,
    ///*Deprecated* Scheduling status of this task for the user it is assigned to. This field can only be set if the assignee is non-null. Setting this field to "inbox" or "upcoming" inserts it at the top of the section, while the other options will insert at the bottom.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_status: Option<String>,
    ///True if the task is currently marked complete, false if not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    ///The time at which this task was completed, or null if the task is incomplete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by: Option<serde_json::Value>,
    ///The time at which this resource was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    ///[Opt In](/docs/inputoutput-options). Array of resources referencing tasks that this task depends on. The objects contain only the gid of the dependency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<AsanaResource>>,
    ///[Opt In](/docs/inputoutput-options). Array of resources referencing tasks that depend on this task. The objects contain only the ID of the dependent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependents: Option<Vec<AsanaResource>>,
    ///The UTC date and time on which this task is due, or null if the task has no due time. This takes an ISO 8601 date string in UTC and should not be used together with `due_on`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<chrono::DateTime<chrono::Utc>>,
    ///The localized date on which this task is due, or null if the task has no due date. This takes a date with `YYYY-MM-DD` format and should not be used together with `due_at`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on: Option<chrono::NaiveDate>,
    #[doc = "*OAuth Required*. *Conditional*. This field is returned only if external values are set or included by using [Opt In] (/docs/inputoutput-options).\nThe external field allows you to store app-specific metadata on tasks, including a gid that can be used to retrieve tasks and a data blob that can store app-specific character strings. Note that you will need to authenticate with Oauth to access or modify this data. Once an external gid is set, you can use the notation `external:custom_gid` to reference your object anywhere in the API where you may use the original object gid. See the page on Custom External Data for more details."]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<serde_json::Value>,
    ///*Deprecated - please use liked instead* True if the task is hearted by the authorized user, false if not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hearted: Option<bool>,
    ///*Deprecated - please use likes instead* Array of likes for users who have hearted this task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hearts: Option<Vec<Like>>,
    ///[Opt In](/docs/inputoutput-options). The notes of the text with formatting as HTML.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_notes: Option<String>,
    ///[Opt In](/docs/inputoutput-options). In some contexts tasks can be rendered as a visual separator; for instance, subtasks can appear similar to [sections](/reference/sections) without being true `section` objects. If a `task` object is rendered this way in any context it will have the property `is_rendered_as_separator` set to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rendered_as_separator: Option<bool>,
    ///True if the task is liked by the authorized user, false if not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked: Option<bool>,
    ///Array of likes for users who have liked this task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<Vec<Like>>,
    ///*Create-only*. Array of projects this task is associated with and the section it is in. At task creation time, this array can be used to add the task to specific sections. After task creation, these associations can be modified using the `addProject` and `removeProject` endpoints. Note that over time, more types of memberships may be added to this property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memberships: Option<Vec<serde_json::Value>>,
    /**The time at which this task was last modified.

The following conditions will change `modified_at`:

- story is created on a task
- story is trashed on a task
- attachment is trashed on a task
- task is assigned or unassigned
- custom field value is changed
- the task itself is trashed
- Or if any of the following fields are updated:
  - completed
  - name
  - due_date
  - description
  - attachments
  - items
  - schedule_status

The following conditions will _not_ change `modified_at`:

- moving to a new container (project, portfolio, etc)
- comments being added to the task (but the stories they generate
  _will_ affect `modified_at`)*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    ///Name of the task. This is generally a short sentence fragment that fits on a line in the UI for maximum readability. However, it can be longer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Free-form textual information associated with the task (i.e. its description).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    ///*Deprecated - please use likes instead* The number of users who have hearted this task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_hearts: Option<i64>,
    ///The number of users who have liked this task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_likes: Option<i64>,
    ///[Opt In](/docs/inputoutput-options). The number of subtasks on this task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_subtasks: Option<i64>,
    /**Date and time on which work begins for the task, or null if the task has no start time. This takes an ISO 8601 date string in UTC and should not be used together with `start_on`.
*Note: `due_at` must be present in the request when setting or unsetting the `start_at` parameter.**/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
    /**The day on which work begins for the task , or null if the task has no start date. This takes a date with `YYYY-MM-DD` format and should not be used together with `start_at`.
*Note: `due_on` or `due_at` must be present in the request when setting or unsetting the `start_on` parameter.**/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_on: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for TaskBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TaskBase {
    type Target = TaskCompact;
    fn deref(&self) -> &Self::Target {
        &self.task_compact
    }
}
impl std::ops::DerefMut for TaskBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.task_compact
    }
}