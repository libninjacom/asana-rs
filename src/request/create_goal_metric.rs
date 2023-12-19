use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::create_goal_metric`].

On request success, this will return a [`CreateGoalMetricResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGoalMetricRequest {
    pub data: GoalMetricRequest,
    pub goal_gid: String,
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
}
impl CreateGoalMetricRequest {}
impl FluentRequest<'_, CreateGoalMetricRequest> {
    pub fn opt_fields(
        mut self,
        opt_fields: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .opt_fields = Some(
            opt_fields.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreateGoalMetricRequest> {
    type Output = httpclient::InMemoryResult<CreateGoalMetricResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/goals/{goal_gid}/setMetric", goal_gid = self.params.goal_gid
            );
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "data" : self.params.data }));
            if let Some(ref unwrapped) = self.params.opt_fields {
                for item in unwrapped {
                    r = r.query("opt_fields[]", &item.to_string());
                }
            }
            if let Some(ref unwrapped) = self.params.opt_pretty {
                r = r.query("opt_pretty", &unwrapped.to_string());
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}