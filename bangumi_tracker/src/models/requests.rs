use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct TorrentSearchQuery {
    pub query: String,
    pub page_index: Option<u64>,
    pub page_size: Option<u64>,
}
