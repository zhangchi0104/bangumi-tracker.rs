use crate::models::common::Torrent;
use kei_derive::Responder;
use serde::{Deserialize, Serialize};
use crate::downloaders::aria2::Aria2Response;

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
#[serde(rename_all = "camelCase")]
pub struct TaskStatusResponse {
    pub completed_length: u128,
    pub total_length: u128,
    pub download_speed: u128,
    pub dir: String,
    pub files: Vec<DownloadFileInfo>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename = "camelCase")]
pub struct DownloadFileInfo {
    pub completed_length: u128,
    #[serde(rename(deserialize = "length"))]
    pub total_length: u128,
    pub path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Responder)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct AddDownloadTaskResponse {
    pub id: String,
}

