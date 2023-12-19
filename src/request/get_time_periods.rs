use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::get_time_periods`].

On request success, this will return a [`GetTimePeriodsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTimePeriodsRequest {
    pub end_on: Option<chrono::NaiveDate>,
    pub limit: Option<i64>,
    pub offset: Option<String>,
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
    pub start_on: Option<chrono::NaiveDate>,
    pub workspace: String,
}
impl GetTimePeriodsRequest {}
impl FluentRequest<'_, GetTimePeriodsRequest> {
    pub fn end_on(mut self, end_on: chrono::NaiveDate) -> Self {
        self.params.end_on = Some(end_on);
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
    pub fn start_on(mut self, start_on: chrono::NaiveDate) -> Self {
        self.params.start_on = Some(start_on);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetTimePeriodsRequest> {
    type Output = httpclient::InMemoryResult<GetTimePeriodsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/time_periods";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
