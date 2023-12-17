use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::trigger_rule`].

On request success, this will return a [`TriggerRuleResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerRuleRequest {
    pub data: Option<RuleTriggerRequest>,
    pub rule_trigger_gid: String,
}
impl TriggerRuleRequest {}
impl FluentRequest<'_, TriggerRuleRequest> {
    pub fn data(mut self, data: RuleTriggerRequest) -> Self {
        self.params.data = Some(data);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TriggerRuleRequest> {
    type Output = httpclient::InMemoryResult<TriggerRuleResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/rule_triggers/{rule_trigger_gid}/run", rule_trigger_gid = self.params
                .rule_trigger_gid
            );
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.data {
                r = r.json(json!({ "data" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}