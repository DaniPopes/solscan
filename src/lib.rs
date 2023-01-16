//! # solscan
//!
//! Rust [Solscan](https://solscan.io) API client.

mod response;
use response::Response;
pub use response::{ClientError, ResponseError, ResponseErrorMessage, Result};

mod block;
pub use block::*;

mod transaction;
pub use transaction::*;

mod account;
pub use account::*;

mod token;
pub use token::*;

mod other;
pub use other::*;

use reqwest::{header, Client as RClient, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};

pub use reqwest::{self, Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder, IntoUrl};
pub use url::Url;

/// The [Solscan API URL](https://public-api.solscan.io/docs).
pub const BASE_URL: &str = "https://public-api.solscan.io/";

/// A [Solscan API](https://public-api.solscan.io/docs) client.
#[derive(Clone, Debug)]
pub struct Client {
    client: RClient,
    base_url: Url,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// Instantiate a new client with the [base URL][BASE_URL].
    pub fn new() -> Self {
        Self::with_url(BASE_URL).unwrap()
    }

    /// Instantiate a new client with the provided URL.
    pub fn with_url(url: impl IntoUrl) -> Result<Self> {
        Self::with_url_and_client(url, RClient::new())
    }

    /// Instantiate a new client with the provided URL and reqwest client.
    pub fn with_url_and_client(url: impl IntoUrl, client: RClient) -> Result<Self> {
        Ok(Self { client, base_url: url.into_url()? })
    }

    /// Performs an HTTP `GET` request.
    pub async fn get<T: DeserializeOwned, Q: Serialize + ?Sized>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<T> {
        self._get(path)?.query(query).send().await?.json::<Response<T>>().await?.result()
    }

    /// Performs an HTTP `GET` request without a query string.
    pub async fn get_no_query<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self._get(path)?.send().await?.json::<Response<T>>().await?.result()
    }

    fn _get(&self, path: &str) -> Result<RequestBuilder> {
        let url = self.base_url.join(path)?;
        Ok(self.client.get(url).header(header::ACCEPT, "application/json"))
    }
}

#[inline(always)]
fn make_path1(base_path: &'static str, value: &str) -> String {
    let mut s = String::with_capacity(base_path.len() + value.len());
    s.push_str(base_path);
    s.push_str(value);
    s
}

#[allow(dead_code)]
mod serde_string {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::{fmt::Display, str::FromStr};

    pub fn serialize<T: ToString, S: Serializer>(value: T, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, E, T, D>(d: D) -> Result<T, D::Error>
    where
        T: FromStr<Err = E>,
        E: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(d).and_then(|s| T::from_str(&s).map_err(serde::de::Error::custom))
    }

    pub mod option {
        use super::*;

        pub fn serialize<T: ToString, S: Serializer>(
            value: Option<T>,
            s: S,
        ) -> Result<S::Ok, S::Error> {
            if let Some(value) = value {
                s.serialize_some(&value.to_string())
            } else {
                s.serialize_none()
            }
        }

        pub fn deserialize<'de, E, T, D>(d: D) -> Result<Option<T>, D::Error>
        where
            T: FromStr<Err = E>,
            E: Display,
            D: Deserializer<'de>,
        {
            match <Option<String>>::deserialize(d) {
                Ok(Some(s)) => {
                    if s.is_empty() {
                        Ok(None)
                    } else {
                        match T::from_str(&s) {
                            Ok(x) => Ok(Some(x)),
                            Err(e) => Err(serde::de::Error::custom(e)),
                        }
                    }
                }
                Ok(None) => Ok(None),
                Err(e) => Err(e),
            }
        }
    }

    pub mod vec {
        use super::*;
        use serde::ser::SerializeSeq;

        pub fn serialize<T: ToString, S: Serializer>(
            value: Vec<T>,
            s: S,
        ) -> Result<S::Ok, S::Error> {
            let mut seq = s.serialize_seq(Some(value.len()))?;
            for value in value {
                seq.serialize_element(&value.to_string())?;
            }
            seq.end()
        }

        pub fn deserialize<'de, E, T, D>(d: D) -> Result<Vec<T>, D::Error>
        where
            T: FromStr<Err = E>,
            E: Display,
            D: Deserializer<'de>,
        {
            <Vec<String>>::deserialize(d).and_then(|v| {
                v.into_iter()
                    .map(|s| T::from_str(&s))
                    .collect::<Result<Vec<T>, _>>()
                    .map_err(serde::de::Error::custom)
            })
        }
    }
}
