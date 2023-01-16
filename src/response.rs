//! Solscan API response

use serde_json::Value;
use thiserror::Error;

/// Type alias for Result<T, ClientError>
pub type Result<T = ()> = std::result::Result<T, ClientError>;

/// A Solscan error.
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Solscan error {}: {}", _0.status, _0.error.message)]
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

api_models! {
    pub struct ResponseErrorMessage {
        pub message: String,
    }

    pub struct ResponseError {
        pub status: i32,
        pub error: ResponseErrorMessage,
    }

    #[serde(untagged)]
    pub(crate) enum Response<T> {
        Ok(T),
        Err(ResponseError),
        Unknown(Value),
        #[default]
        Fallback
    }
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
            _ => Err(ClientError::EmptyResponse),
        }
    }
}
