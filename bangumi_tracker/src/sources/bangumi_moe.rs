use crate::errors::LogError;
use crate::models::responses::BangumiSearchResponse;
use crate::Result;
use crate::{errors::Error, models::responses::CurrentSeasonBangumiResponse};

pub async fn fetch_current_season() -> Result<CurrentSeasonBangumiResponse> {
    const URL: &str = "https://bangumi.moe/api/v2/bangumi/current";
    reqwest::get(URL)
        .await
        .map_err(|_| Error::ApiClientError(Some(URL)))?
        .json()
        .await
        .map_logged_err(|_| {
            Error::SerializeError(std::any::type_name::<CurrentSeasonBangumiResponse>())
        })
}

pub async fn search(
    keyword: &str,
    page_size: Option<u64>,
    page_index: Option<u64>,
) -> Result<BangumiSearchResponse> {
    let query_page_size = page_size.unwrap_or(10);
    let query_page_index = page_index.unwrap_or(0);
    let request_url = format!(
        "https://bangumi.moe/api/v2/torrent/search?p={}&limit={}&query={}",
        query_page_index, query_page_size, keyword
    );
    reqwest::get(request_url)
        .await
        .map_logged_err(|_| {
            Error::ApiClientError(Some("https://bangumi.moe/api/v2/torrent/search"))
        })?
        .json()
        .await
        .map_logged_err(|_| {
            Error::SerializeError(std::any::type_name::<CurrentSeasonBangumiResponse>())
        })
}
