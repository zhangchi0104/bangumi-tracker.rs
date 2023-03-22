use std::ops::Deref;

use crate::Result;
use crate::{models::responses::AddDownloadTaskResponse, FromEnv, FromEnvDefault};
mod aria2;

pub enum Downloader<'downloader> {
    Aria2(aria2::Aria2Client<'downloader>),
}

impl Downloader<'_> {
    pub async fn add_task(&self, url: &str) -> Result<AddDownloadTaskResponse> {
        Ok(match self {
            Downloader::Aria2(client) => client.add_task(url).await?,
        })
    }
}
impl Default for Downloader<'_> {
    fn default() -> Self {
        Downloader::Aria2(aria2::Aria2Client::default())
    }
}

impl FromEnv for Downloader<'_> {
    fn from_env() -> Result<Self, std::env::VarError> {
        let client = aria2::Aria2Client::from_env()?;
        Ok(Downloader::Aria2(client))
    }
}

impl FromEnvDefault for Downloader<'_> {}
