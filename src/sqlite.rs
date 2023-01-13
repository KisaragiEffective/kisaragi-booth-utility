use std::num::NonZeroUsize;
use std::path::Path;
use sqlite3::Error;
use thiserror::Error;
use crate::{Browser, ExecutionError, GetAuthorizationTokenError};

#[derive(Error, Debug)]
#[error("sqlite3 error (code {code:?}): {message:?}")]
pub struct SQLite3ErrorWithCompare {
    code: Option<isize>,
    message: Option<String>,
}

impl PartialEq<Self> for SQLite3ErrorWithCompare {
    fn eq(&self, other: &Self) -> bool {
        let s = &self;
        let o = &other;

        s.code == o.code && s.message == o.message
    }
}

impl Eq for SQLite3ErrorWithCompare {}

impl From<Error> for SQLite3ErrorWithCompare {
    fn from(value: Error) -> Self {
        Self {
            message: value.message,
            code: value.code,
        }
    }
}

pub(crate) fn it(cookie_file: impl AsRef<Path>, browser: Browser) -> Result<(), ExecutionError> {
    let cookie_file = cookie_file.as_ref();

    if !cookie_file.exists() {
        return Err(ExecutionError::CommandLineArgumentValidation("--cookie-file must point to existing path".to_string()))
    }

    if cookie_file.is_dir() {
        return Err(ExecutionError::CommandLineArgumentValidation("--cookie-file must point to file".to_string()))
    }

    let call_sql = || {
        match browser {
            Browser::Firefox => r#"select value from moz_cookies where host = '.booth.pm' and name = '_plaza_session_nktz7u';"#,
            Browser::Chromium => r#"select value from cookies where host = '.booth.pm' and name = '_plaza_session_nktz7u'"#,
            Browser::UnsupportedBrowser(_) => unreachable!()
        }
    };
    let handle = || {
        let temp_file = tempfile::NamedTempFile::new()?;
        std::fs::copy(&cookie_file, temp_file.path())?;
        let s3 = sqlite3::open(temp_file.path()).map_err(SQLite3ErrorWithCompare::from)?;
        let mut rows = vec![];
        s3.iterate(call_sql(), |row| {
            let x = row.iter().map(|(_, v)| v.unwrap()).collect::<Vec<_>>().join("\t");
            rows.push(x);
            true
        }).map_err(SQLite3ErrorWithCompare::from)?;

        temp_file.close()?;

        Ok::<_, ExecutionError>(rows)
    };

    let records = match browser {
        Browser::Firefox => {
            handle()?
        }
        Browser::Chromium => {
            handle()?
        }
        Browser::UnsupportedBrowser(browser) => {
            return Err(ExecutionError::CommandLineArgumentValidation(format!("{browser} is not supported yet.")))
        }
    };

    if records.is_empty() {
        Err(ExecutionError::GetAuthorizationToken(GetAuthorizationTokenError::NotFound))
    } else if records.len() >= 2 {
        Err(ExecutionError::GetAuthorizationToken(GetAuthorizationTokenError::MultipleTokensFound {
            // SAFETY: just known
            count: unsafe { NonZeroUsize::new_unchecked(records.len()) }
        }))
    } else {
        let record = &records[0];
        println!("{record}");

        Ok(())
    }
}
