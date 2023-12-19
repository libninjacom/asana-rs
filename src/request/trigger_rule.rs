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
    pub data: RuleTriggerRequest,
    pub rule_trigger_gid: String,
}
impl TriggerRuleRequest {}
impl FluentRequest<'_, TriggerRuleRequest> {}
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
            r = r.json(json!({ "data" : self.params.data }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}