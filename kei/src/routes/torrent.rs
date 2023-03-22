use crate::{
    derive_configure,
    models::{requests::TorrentSearchQuery, responses::BangumiSearchResponse},
    state::AppState,
    Result,
};
use actix_web::{get, web};
#[get("/search")]
async fn search_torrent(
    state: web::Data<AppState<'_>>,
    query: web::Query<TorrentSearchQuery>,
) -> Result<BangumiSearchResponse> {
    state
        .bangumi_source
        .search_torrent(query.query.as_str(), query.page_index, query.page_size)
        .await
}
derive_configure!("/torrent", search_torrent);
