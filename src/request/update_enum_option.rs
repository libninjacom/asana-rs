use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::update_enum_option`].

On request success, this will return a [`UpdateEnumOptionResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEnumOptionRequest {
    pub data: Option<EnumOptionBase>,
    pub enum_option_gid: String,
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
}
impl UpdateEnumOptionRequest {}
impl FluentRequest<'_, UpdateEnumOptionRequest> {
    pub fn data(mut self, data: EnumOptionBase) -> Self {
        self.params.data = Some(data);
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
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, UpdateEnumOptionRequest> {
    type Output = httpclient::InMemoryResult<UpdateEnumOptionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/enum_options/{enum_option_gid}", enum_option_gid = self.params
                .enum_option_gid
            );
            let mut r = self.client.client.put(url);
            if let Some(ref unwrapped) = self.params.data {
                r = r.json(json!({ "data" : unwrapped }));
            }
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