use std::ops::Deref;

use crate::{downloaders::Downloader, sources::BangumiSource, FromEnv, FromEnvDefault};
pub struct AppState<'state> {
    pub(crate) bangumi_source: BangumiSource,
    pub(crate) downloader: Downloader<'state>,
}

impl Default for AppState<'_> {
    fn default() -> Self {
        Self {
            bangumi_source: BangumiSource::BangumiMoe,
            downloader: Downloader::default(),
        }
    }
}

impl AppState<'_> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FromEnv for AppState<'_> {
    fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            bangumi_source: BangumiSource::from_env()?,
            downloader: Downloader::from_env()?,
        })
    }
}
impl FromEnvDefault for AppState<'_> {}
