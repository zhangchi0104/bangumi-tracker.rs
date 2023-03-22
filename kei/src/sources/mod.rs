use crate::models::responses::{BangumiSearchResponse, CurrentSeasonBangumiResponse};
use crate::{FromEnv, FromEnvDefault, Result};

mod bangumi_moe;

pub enum BangumiSource {
    BangumiMoe,
}
impl Default for BangumiSource {
    fn default() -> Self {
        BangumiSource::BangumiMoe
    }
}

impl BangumiSource {
    pub async fn fetch_current_season_bangumi(&self) -> Result<CurrentSeasonBangumiResponse> {
        match self {
            BangumiSource::BangumiMoe => bangumi_moe::fetch_current_season(),
        }
        .await
    }

    pub async fn search_torrent(
        &self,
        query: &str,
        page_index: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<BangumiSearchResponse> {
        match self {
            BangumiSource::BangumiMoe => bangumi_moe::search(query, page_size, page_index),
        }
        .await
    }
}
impl FromEnv for BangumiSource {
    fn from_env() -> std::result::Result<Self, std::env::VarError> {
        let source_str = std::env::var("KEI_BANGUMI_SOURCE")?;
        let source = match source_str.as_str() {
            _ => BangumiSource::BangumiMoe,
        };
        Ok(source)
    }
}

impl FromEnvDefault for BangumiSource {}
