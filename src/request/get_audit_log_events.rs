use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::get_audit_log_events`].

On request success, this will return a [`GetAuditLogEventsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuditLogEventsRequest {
    pub actor_gid: Option<String>,
    pub actor_type: Option<String>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
    pub event_type: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<String>,
    pub resource_gid: Option<String>,
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
    pub workspace_gid: String,
}
impl GetAuditLogEventsRequest {}
impl FluentRequest<'_, GetAuditLogEventsRequest> {
    pub fn actor_gid(mut self, actor_gid: &str) -> Self {
        self.params.actor_gid = Some(actor_gid.to_owned());
        self
    }
    pub fn actor_type(mut self, actor_type: &str) -> Self {
        self.params.actor_type = Some(actor_type.to_owned());
        self
    }
    pub fn end_at(mut self, end_at: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.end_at = Some(end_at);
        self
    }
    pub fn event_type(mut self, event_type: &str) -> Self {
        self.params.event_type = Some(event_type.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.params.limit = Some(limit);
        self
    }
    pub fn offset(mut self, offset: &str) -> Self {
        self.params.offset = Some(offset.to_owned());
        self
    }
    pub fn resource_gid(mut self, resource_gid: &str) -> Self {
        self.params.resource_gid = Some(resource_gid.to_owned());
        self
    }
    pub fn start_at(mut self, start_at: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.start_at = Some(start_at);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetAuditLogEventsRequest> {
    type Output = httpclient::InMemoryResult<GetAuditLogEventsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/workspaces/{workspace_gid}/audit_log_events", workspace_gid = self
                .params.workspace_gid
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}