#[derive(Deserialize)]
#[serde(untagged)]
pub enum UploadResult {
    Ok {
        /// 過去にアップロードされたファイル。`uploaded_file`を**含まない**。
        #[serde(rename = "files")]
        uploaded_in_past: Vec<BoothUploadedObject>,
        /// アップロードが完了した時点の容量情報
        storage: StorageQuota,
        /// 現在アップロードしたファイル
        #[serde(rename = "file")]
        uploaded_file: BoothUploadedObject,
    },
    Err(UploadError),
}

#[derive(Deserialize, Error, Debug)]
#[serde(untagged)]
pub enum UploadError {
    #[error("multiple remote rejection: {errors}")]
    Aggregate {
        #[from]
        errors: InnerAggregateError,
    },
    #[error("remote error: {error}")]
    Single {
        #[from]
        error: InnerError,
    },
    #[error("unable to obtain CSRF token")]
    #[serde(skip)]
    UnableToObtainCsrfToken,
}

#[derive(Deserialize, Error, Debug)]
#[error("downloadble: {downloadable:?}")]
pub struct InnerAggregateError {
    #[serde(default)]
    downloadable: Option<DownloadableError>,
}

#[derive(Deserialize, Error, Debug)]
#[error("downloadble: {file:?}")]
pub struct DownloadableError {
    #[from]
    file: DownloadableContentError,
}

// ファイル形式のエラー
// ファイルが空のエラー
// サイズが過小のエラー
#[derive(Deserialize, Error, Debug)]
#[error("{0:?}")]
pub struct DownloadableContentError(Vec<String>);

// Unauthorizedなエラー
#[derive(Deserialize, Error, Debug)]
#[error("{0:?}")]
pub struct InnerError(String);

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;
use thiserror::Error;

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