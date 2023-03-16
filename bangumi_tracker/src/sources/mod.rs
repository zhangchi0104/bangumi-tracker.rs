use crate::models::responses::{BangumiSearchResponse, CurrentSeasonBangumiResponse};
use crate::Result;

mod bangumi_moe;

pub enum BangumiSource {
    BangumiMoe,
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
