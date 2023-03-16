use chrono::{DateTime, Utc};
use chrono::serde::ts_milliseconds;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Bangumi {
    #[serde(rename(deserialize = "_id"))]
    id: String,
    name: String,
    tag_id: String,
    credit: String,
    #[serde(rename(deserialize = "startDate"), with = "ts_milliseconds")]
    start_date: DateTime<Utc>,
    #[serde(rename(deserialize = "endDate"), with = "ts_milliseconds")]
    end_date: DateTime<Utc>,
    #[serde(alias = "showOn")]
    show_on: u8,
    icon: String,
    cover: String,
    tag: BangumiTag,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct BangumiTag {
    #[serde(rename(deserialize = "_id"))]
    id: String,
    name: String,
    locale: std::collections::HashMap<String, String>,
    #[serde(alias = "synonyms")]
    aliases: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ReleaseGroup {
    #[serde(rename(deserialize = "_id"))]
    id: String,
    name: String,
    tag_id: String,
    icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Torrent {
    #[serde(rename(deserialize = "_id"))]
    id: String,
    title: String,
    #[serde(rename(deserialize = "downloads"))]
    num_downloads: u64,
    #[serde(rename(deserialize = "finished"))]
    num_finished: u64,
    magnet: String,
    tag_ids: Vec<String>,
    content: Vec<(String, String)>,
}
