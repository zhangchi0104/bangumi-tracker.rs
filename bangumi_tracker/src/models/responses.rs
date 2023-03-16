use crate::models::common::Torrent;
use bangumi_tracker_derive::Responder;
use serde::{Deserialize, Serialize};

use super::common::{Bangumi, ReleaseGroup};

#[derive(Deserialize, Serialize, Debug, Clone, Responder)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CurrentSeasonBangumiResponse {
    pub bangumis: Vec<Bangumi>,
    pub working_teams: std::collections::HashMap<String, Vec<ReleaseGroup>>,
}

#[derive(Deserialize, Debug, Serialize, Clone, Responder)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct BangumiSearchResponse {
    torrents: Vec<Torrent>,
    count: u64,
    #[serde(rename(deserialize = "page_count"))]
    num_pages: u64,
    success: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Responder)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct AddDownloadTaskResponse {
    #[serde(rename(deserialize = "result"))]
    pub gid: String,
}
