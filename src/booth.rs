#[derive(Deserialize)]
#[serde(untagged)]
pub enum UploadResponse {
    Ok {
        files: Vec<BoothUploadedObject>,
        storage: StorageQuota,
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
pub struct StorageQuota {
    #[serde(rename = "storage_quota")]
    pub quota: usize,
    #[serde(rename = "storage_usage")]
    pub usage: usize,
}

impl StorageQuota {
    pub fn left(&self) -> usize {
        self.quota - self.usage
    }
}

#[derive(Deserialize)]
pub struct BoothUploadedObject {
    id: FileId,
    item_id: ItemId,
    file: OpaqueFile,
    pub file_size: usize,
    download_count: usize,
    crated_at: JapanStandardTime,
    updated_at: JapanStandardTime,
    deleted_at: Option<JapanStandardTime>,
    display_order: usize,
    pub name: String,
}