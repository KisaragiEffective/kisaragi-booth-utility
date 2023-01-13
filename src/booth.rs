#[derive(Deserialize)]
#[serde(untagged)]
pub enum UploadResponse {
    Ok {
        files: Vec<BoothUploadedObject>,
        storage: DiskQuota,
        file: BoothUploadedObject,
    },
    Err {
        error: String,
    }
}

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

type JapanStandardTime = DateTime<FixedOffset>;

#[derive(Deserialize)]
pub struct FileId(u32);

#[derive(Deserialize)]
pub struct ItemId(u32);

#[derive(Deserialize)]
pub struct OpaqueFile {
    pub filename: String,
}

#[derive(Deserialize)]
pub struct DiskQuota {
    #[serde(rename = "disk_quota")]
    pub quota: usize,
    #[serde(rename = "disk_usage")]
    pub usage: usize,
}

impl DiskQuota {
    pub fn left(&self) -> usize {
        self.quota - self.usage
    }
}

#[derive(Deserialize)]
pub struct BoothUploadedObject {
    item_id: ItemId,
    pub file_size: usize,
    pub name: String,
}

const fn none<T>() -> Option<T> {
    None
}
