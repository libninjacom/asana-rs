use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::delete_time_tracking_entry`].

On request success, this will return a [`DeleteTimeTrackingEntryResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTimeTrackingEntryRequest {
    pub opt_pretty: Option<bool>,
    pub time_tracking_entry_gid: String,
}
impl DeleteTimeTrackingEntryRequest {}
impl FluentRequest<'_, DeleteTimeTrackingEntryRequest> {
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DeleteTimeTrackingEntryRequest> {
    type Output = httpclient::InMemoryResult<DeleteTimeTrackingEntryResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/time_tracking_entries/{time_tracking_entry_gid}",
                time_tracking_entry_gid = self.params.time_tracking_entry_gid
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
