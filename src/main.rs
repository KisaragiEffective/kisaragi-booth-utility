#![deny(clippy::all, clippy::perf)]
#![warn(clippy::nursery, clippy::pedantic)]

mod pretty_size;
mod booth;
mod sqlite;

use std::num::NonZeroUsize;
use std::path::PathBuf;
use clap::Parser;
use reqwest::multipart::{Form, Part};
use select::predicate::Predicate;
use strum::EnumString;
use thiserror::Error;
use crate::booth::{UploadError, UploadResult};
use crate::sqlite::SQLite3ErrorWithCompare;

/// Utility around booth.pm, developed by Kisaragi Marine.
/// This project is not related, developed, nor affiliated by pixiv inc.
/// Please refer to <https://policies.pixiv.net/> and <https://policies.pixiv.net/#booth>
/// before use.
#[derive(Parser)]
#[command(author, version, about, long_about)]
enum CommandLineSubCommand {
    GetAuthorizationToken {
        #[clap(short, long)]
        /// Path to `cookies.sqlite` if firefox, `Cookies` if chromium.
        cookie_file: PathBuf,
        #[clap(short, long)]
        /// accepts `firefox` or `chromium`.
        /// Internet Explorer, Safari, Sleipnir, Lunaspace, legacy Edge and legacy Opera are unsupported.
        browser: Browser,
    },
    Upload {
        #[clap(short = 'i', long)]
        /// Your item's id. e.g. https://booth.pm/ja/items/3519955 -> 3519955
        booth_item_id: i32,
        #[clap(short = 'p', long)]
        /// Your local path to be uploaded.
        artifact_path: PathBuf,
        #[clap(short = 't', long, long = "token")]
        /// Can be grabbed by `get-authorization-token` subcommand.
        login_token: String,
        #[clap(long)]
        /// Sets `Accept-Language` in HTTP request, sending its value from your environment
        /// variable to localize error to your language.
        ///
        /// This flag does not have effect on non-*nix platform.
        localize_remote_error: bool,
        #[clap(long)]
        /// UNSAFE: Displays X-CSRF-Token to stdout.
        unsafe_expose_csrf_token: bool,
        #[clap(long)]
        /// UNSAFE: prints ALL header, including `cookie` header.
        /// Only intended usage is debug purpose.
        unsafe_expose_all_header: bool,
    },
}

#[derive(Error, Debug)]
pub(crate) enum ExecutionError {
    #[error("Database error occured: {0}")]
    Database(#[from] SQLite3ErrorWithCompare),
    #[error("Incorrect usage of command line argument: {0}")]
    CommandLineArgumentValidation(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Error occurred during fetching authorization token:")]
    GetAuthorizationToken(#[from] GetAuthorizationTokenError),
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("booth remote server error: {0}")]
    BoothUploadError(#[from] UploadError),
}

#[derive(Error, Debug)]
enum GetAuthorizationTokenError {
    #[error("No tokens found")]
    NotFound,
    #[error("Multiple tokens (size: {count}) found")]
    MultipleTokensFound {
        count: NonZeroUsize,
    },
}

#[derive(EnumString, Debug, Clone, Eq, PartialEq)]
pub(crate) enum Browser {
    #[strum(serialize = "firefox")]
    Firefox,
    #[strum(serialize = "chrome", serialize = "chromium", serialize = "vivaldi", serialize = "opera", serialize = "edge")]
    Chromium,
    #[strum(default)]
    UnsupportedBrowser(String),
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::similar_names)]
#[tokio::main]
async fn main() -> Result<(), ExecutionError> {
    let clsc = CommandLineSubCommand::parse();
    match clsc {
        CommandLineSubCommand::GetAuthorizationToken { cookie_file, browser } => {
            sqlite::it(cookie_file, browser)?;
        }
        CommandLineSubCommand::Upload {
            booth_item_id,
            artifact_path,
            login_token,
            localize_remote_error,
            unsafe_expose_csrf_token,
            unsafe_expose_all_header,
        } => {
            if !artifact_path.exists() {
                return Err(ExecutionError::CommandLineArgumentValidation("--artifact-path must point to existing path".to_string()))
            }

            if artifact_path.is_dir() {
                return Err(ExecutionError::CommandLineArgumentValidation("--artifact-path must point to file".to_string()))
            }

            let upload_url = format!("https://manage.booth.pm/items/{booth_item_id}/downloadables/");
            eprintln!("url: {url}", url = &upload_url);
            eprintln!("from: `{p}`", p = &artifact_path.display());

            // reqwestのJarがなぜかcookieを渡さないので主導でmanipulateする
            let baked_cookie = format!("_plaza_session_nktz7u={v}", v = &login_token);

            let csrf_token = {
                let client = reqwest::ClientBuilder::new()
                    .gzip(true)
                    .build()
                    .unwrap();

                // X-CSRF-Token対策
                println!("Getting CSRF token");
                let top_page = client.get(format!("https://manage.booth.pm/items/{booth_item_id}/edit"))
                    .header("Accept", "text/html; charset=utf-8")
                    .header("User-Agent", "KisaragiEffective/booth-upload-ci")
                    .header("Cookie", &baked_cookie)
                    .send()
                    .await?
                    .text()
                    .await?;

                let doc = select::document::Document::from(&*top_page);
                let csrf_opt = doc
                    .find(select::predicate::Name("meta").and(select::predicate::Attr("name", "csrf-token")))
                    .find_map(|x| x.attr("content"));

                if let Some(csrf) = csrf_opt {
                    let csrf = csrf.to_owned();
                    if unsafe_expose_csrf_token {
                        println!("[CSRF] {csrf}");
                    }
                    csrf
                } else {
                    return Err(ExecutionError::BoothUploadError(UploadError::UnableToObtainCsrfToken))
                }
            };


            let form = {
                let form = Form::default();
                let bytes = std::fs::read(&artifact_path)?;
                let file_name = artifact_path.file_name()
                    .map(|x| x.to_str().unwrap().to_string())
                    .expect("upload file must have name");
                // mime is inferred by remote
                let upload = Part::bytes(bytes)
                    .file_name(file_name);

                form.part("downloadable[file]", upload)
            };

            let client = reqwest::ClientBuilder::new()
                .gzip(true)
                .build()
                .unwrap();

            let mut req = client.post(upload_url)
                .multipart(form)
                .header("Accept", "application/json")
                .header("User-Agent", "KisaragiEffective/booth-upload-ci")
                .header("Cookie", &baked_cookie)
                // 欠けているとリクエストが正しくても422
                .header("X-CSRF-Token", csrf_token);

            cfg_if::cfg_if! {
                if #[cfg(unix)] {
                    if let Some(language_preference) = std::env::var_os("LANG") {
                        use std::os::unix::ffi::OsStrExt;
                        if localize_remote_error && language_preference.as_bytes().starts_with(b"ja_JP") {
                            req = req.header("Accept-Language", "ja");
                        }
                    }
                }
            }

            let res = req
                .send()
                .await?;

            if unsafe_expose_all_header {
                let http_version = res.version();
                let http_status = res.status().as_u16();
                println!("{http_version:?} {http_status}");
                let headers = res.headers();
                for (name, value) in headers {
                    let value = if value.is_sensitive() {
                        "《redacted》"
                    } else {
                        value.to_str().expect("received garbage in headers from remote server")
                    };
                    println!("{name}: {value}", name = name.as_str());
                }
            }


            #[allow(clippy::similar_names)]
            let res = res
                .json::<UploadResult>()
                .await?;

            match res {
                UploadResult::Ok { storage, uploaded_file: file, .. } => {
                    use crate::pretty_size::pretty_size;
                    println!("uploaded as {name} ({size})", name = file.name, size = pretty_size(file.file_size));
                    println!(
                        "quota: (permitted = {permitted}) - (used = {used}) = (left = {left})",
                        permitted = storage.quota,
                        used = storage.usage,
                        left = storage.left()
                    );
                }
                UploadResult::Err(error) => {
                    return Err(error.into())
                }
            }
        }
        /*
        CommandLineSubCommand::ListChoice { booth_item_id } => {
            TODO
        }

         */
    }
    Ok(())
}
