//! Solscan API response

use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

/// Type alias for Result<T, ClientError>
pub type Result<T = ()> = std::result::Result<T, ClientError>;

/// A Solscan error.
#[derive(Debug, Error)]
pub enum ClientError {
    #[error(transparent)]
    Response(ResponseError),

    #[error("Received an empty response")]
    EmptyResponse,

    #[error("Received an unknown response: {0}")]
    UnknownResponse(Value),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),
}

#[derive(Clone, Debug, Deserialize, Error)]
#[error("{message}")]
pub struct ResponseErrorMessage {
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Error)]
#[error("Solscan error {status}: {error}")]
pub struct ResponseError {
    pub status: i32,
    pub error: ResponseErrorMessage,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum Response<T> {
    Ok(T),
    Err(ResponseError),
    Unknown(Value),
}

impl<T> Response<T> {
    pub fn result(self) -> Result<T> {
        match self {
            Self::Ok(x) => Ok(x),
            Self::Err(e) => Err(ClientError::Response(e)),
            Self::Unknown(value) => match value {
                Value::Array(ref arr) if arr.is_empty() => Err(ClientError::EmptyResponse),
                Value::Object(ref obj) if obj.is_empty() => Err(ClientError::EmptyResponse),
                Value::String(ref s) if s.is_empty() => Err(ClientError::EmptyResponse),
                _ => Err(ClientError::UnknownResponse(value)),
            },
        }
    }
}
