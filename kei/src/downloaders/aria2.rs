use crate::{
    errors::{Error, LogError},
    FromEnv,
    FromEnvDefault, models::responses::AddDownloadTaskResponse, Result,
};
use serde_json::json;
use std::borrow::Cow;
use serde::{Deserialize, Serialize};

pub struct Aria2Client<'client> {
    client: reqwest::Client,
    token: Option<Cow<'client, str>>,
    addr: Cow<'client, str>,
}

impl<'client> Default for Aria2Client<'client> {
    fn default() -> Self {
        Self {
            client: Default::default(),
            token: None,
            addr: Cow::Owned("http://127.0.0.1:6800/jsonrpc".to_owned()),
        }
    }
}

impl<'client> Aria2Client<'client> {
    pub fn addr(&self) -> &str {
        match &self.addr {
            Cow::Borrowed(addr) => addr,
            Cow::Owned(addr) => addr.as_str(),
        }
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_ref().map(|cow| match cow {
            Cow::Borrowed(v) => v,
            Cow::Owned(v) => v.as_str(),
        })
    }
}

impl<'client> Aria2Client<'client> {
    pub async fn add_task(&self, url: &str) -> Result<AddDownloadTaskResponse> {
        let data = json!({
            "jsonrpc": "2.0",
            "method":  "aria2.addUri",
            "id": uuid::Uuid::new_v4().to_string(),
            "params": [
                format!("token:{}", self.token().unwrap_or("")),
                [url]
            ]
        });

        self.client
            .post(self.addr())
            .json(&data)
            .send()
            .await
            .map_logged_err(|_| Error::ApiClientError(Some("Aria2")))?
            .json()
            .await
            .map_logged_err(|_| {
                Error::DeserializeError(std::any::type_name::<AddDownloadTaskResponse>())
            })
    }

    pub async fn fetch_task_status<T>(&self, gid: T) -> Result<bool>
    where
        for<'a> T: AsRef<&'a str>,
    {
        let data = json!({
            "jsonrpc": "2.0",
            "method":  "aria2.tellStatus",
            "id": uuid::Uuid::new_v4().to_string(),
            "params": [
                format!("token:{}", self.token().unwrap_or("")),
                gid.as_ref(),
            ]
        });

        self.client
            .post(self.addr())
            .json(&data)
            .send()
            .await
            .map_logged_err(|_| Error::ApiClientError(Some("Aria2")))?
            .json()
            .await
            .map_logged_err(|_| {
                Error::DeserializeError(std::any::type_name::<AddDownloadTaskResponse>())
            })?;
        Ok(true)
    }
}

impl FromEnv for Aria2Client<'_> {
    fn from_env() -> std::result::Result<Self, std::env::VarError> {
        let url = std::env::var("KEI_ARIA2_URL")?;
        let token = std::env::var("KEI_ARIA2_TOKEN").unwrap_or_default();
        let token = if token.is_empty() {
            None
        } else {
            Some(Cow::Owned(token))
        };
        Ok(Self {
            client: reqwest::Client::default(),
            token: token,
            addr: Cow::Owned(url),
        })
    }
}
impl FromEnvDefault for Aria2Client<'_> {}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Aria2Response<T> {
    id: String,
    jsonrpc: String,
    #[serde(bound(deserialize = "T: Deserialize<'de>", serialize = "T: Serialize"))]
    result: T,
}

impl From<Aria2Response<String>> for AddDownloadTaskResponse {
    fn from(value: Aria2Response<String>) -> Self {
        Self { id: value.result }
    }
}
