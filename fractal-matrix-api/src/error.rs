use gio;
use std::io;
use std::time::SystemTimeError;

use serde_json::Value as JsonValue;

#[derive(Debug)]
pub enum Error {
    BackendError,
    CacheError,
    ReqwestError(reqwest::Error),
    MatrixError(JsonValue),
    SendMsgError(String),
    SendMsgRedactionError(String),
    TokenUsed,
    Denied,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestError(err)
    }
}

derror!(url::ParseError, Error::BackendError);
derror!(io::Error, Error::BackendError);
derror!(gio::Error, Error::BackendError);
derror!(regex::Error, Error::BackendError);
derror!(SystemTimeError, Error::BackendError);

derror!(serde_json::Error, Error::CacheError);
