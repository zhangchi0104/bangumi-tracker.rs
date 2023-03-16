use serde_json::json;

use crate::{
    errors::{Error, LogError},
    models::responses::AddDownloadTaskResponse,
    Result,
};
pub struct Aria2Client<'client> {
    client: reqwest::Client,
    token: Option<&'client str>,
    addr: &'client str,
}

impl<'client> Aria2Client<'client> {
    fn from_token<T>(token: T, addr: T) -> Self
    where
        T: AsRef<&'client str>,
    {
        Self {
            client: reqwest::Client::default(),
            token: Some(token.as_ref()),
            addr: addr.as_ref(),
        }
    }
}

impl<'client> Aria2Client<'client> {
    pub async fn add_url<T>(&self, url: T) -> Result<AddDownloadTaskResponse>
    where
        for<'a> T: AsRef<&'a str>,
    {
        let data = json!({
            "jsonrpc": "2.0",
            "method":  "aria2.addUri",
            "id": uuid::Uuid::new_v4().to_string(),
            "params": [
                format!("token:{}", self.token.unwrap_or("")),
                [url.as_ref()]
            ]
        });

        self.client
            .post(self.addr)
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
                format!("token:{}", self.token.unwrap_or("")),
                gid.as_ref(),
            ]
        });

        self.client
            .post(self.addr)
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
