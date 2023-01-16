//! # solscan
//!
//! Rust [Solscan](https://solscan.io) API client.

#[macro_use]
mod macros;

mod serde_string;

mod response;
use response::Response;
pub use response::{ClientError, ResponseError, ResponseErrorMessage, Result};

mod routes;
pub use routes::*;

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
fn concat_1(base_path: &str, value: &str) -> String {
    let mut s = String::with_capacity(base_path.len() + value.len());
    s.push_str(base_path);
    s.push_str(value);
    s
}
