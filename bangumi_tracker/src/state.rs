use crate::{downloaders::Downloader, sources::BangumiSource};
pub struct AppState {
    pub(crate) bangumi_source: BangumiSource,
    pub(crate) downloader: Downloader,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            bangumi_source: BangumiSource::BangumiMoe,
            downloader: Downloader::default(),
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}
