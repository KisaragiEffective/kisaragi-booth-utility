#[derive(Deserialize)]
#[serde(untagged)]
pub enum UploadResult {
    Ok {
        /// 過去にアップロードされたファイル。`uploaded_file`を**含まない**。
        #[serde(rename = "files")]
        uploaded_in_past: Vec<BoothUploadedObject>,
        /// アップロードが完了した時点の容量情報
        storage: DiskQuota,
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
