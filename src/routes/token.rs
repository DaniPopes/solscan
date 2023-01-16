//! Token section.

use crate::{Client, Result};
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use std::fmt;

api_models! {
    pub struct TokenMeta {
        pub symbol: String,
        #[serde(with = "crate::serde_string")]
        pub address: Pubkey,
        pub name: String,
        pub icon: String,
        pub website: String,
        pub twitter: String,
        pub decimals: u64,
        pub coingecko_id: String,
        pub price: f64,
        pub volume: u64,
        #[serde(with = "crate::serde_string::option")]
        pub token_authority: Option<Pubkey>,
        pub supply: String,
        pub r#type: String,
    }

    pub struct TokenHolderData {
        #[serde(with = "crate::serde_string")]
        pub address: Pubkey,
        pub amount: u64,
        pub decimals: u64,
        #[serde(with = "crate::serde_string")]
        pub owner: Pubkey,
        pub rank: u64,
    }

    // TODO: Missing values?
    pub struct TokenListInfo {
        #[serde(alias = "priceUsdt")]
        pub price_ust: u64,
        pub tag: Vec<String>,
        pub token_name: String,
        pub token_symbol: String,
        pub twitter: String,
        pub website: String,
        pub coin_gecko_info: Value,
        pub sol_alpha_volume: f64,
        #[serde(default)]
        pub _id: Option<Value>,
        #[serde(with = "crate::serde_string")]
        pub address: Pubkey,
        pub created_at: String,
        pub decimals: u64,
        pub extensions: Value,
        pub icon: String,
        pub is_violate: bool,
        pub market_cap_rank: u64,
        #[serde(with = "crate::serde_string")]
        pub mint_address: Pubkey,
        pub symbol_has_lower: bool,
        pub updated_at: String,
        #[serde(alias = "holders")]
        pub holder: u64,
        #[serde(rename = "marketCapFD")]
        pub market_cap_fd: f64,
    }

    pub struct List<T> {
        /// The requested items.
        pub data: Vec<T>,
        /// Not the length of the request list, but the amount of all items.
        pub total: u64,
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum SortBy {
    #[default]
    MarketCap,
    Volume,
    Holder,
    Price,
    PriceChange24h,
    PriceChange7d,
    PriceChange14d,
    PriceChange30d,
    PriceChange60d,
    PriceChange200d,
    PriceChange1y,
}

impl Into<&'static str> for SortBy {
    fn into(self) -> &'static str {
        use SortBy::*;
        match self {
            MarketCap => "market_cap",
            Volume => "volume",
            Holder => "holder",
            Price => "price",
            PriceChange24h => "price_change_24h",
            PriceChange7d => "price_change_7d",
            PriceChange14d => "price_change_14d",
            PriceChange30d => "price_change_30d",
            PriceChange60d => "price_change_60d",
            PriceChange200d => "price_change_200d",
            PriceChange1y => "price_change_1y",
        }
    }
}

impl AsRef<str> for SortBy {
    fn as_ref(&self) -> &str {
        (*self).into()
    }
}

impl fmt::Display for SortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.as_ref())
    }
}

impl Client {
    /// Performs an HTTP `GET` request to the `/token/holders` path.
    pub async fn token_holders(
        &self,
        token_address: &Pubkey,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<List<TokenHolderData>> {
        self.get(
            "token/holders",
            &[
                ("tokenAddress", token_address.to_string()),
                ("limit", limit.unwrap_or(10).to_string()),
                ("offset", offset.unwrap_or(0).to_string()),
            ],
        )
        .await
    }

    /// Performs an HTTP `GET` request to the `/token/meta` path.
    pub async fn token_meta(&self, token_address: &Pubkey) -> Result<TokenMeta> {
        self.get("token/meta", &[("tokenAddress", token_address.to_string())]).await
    }

    /// Performs an HTTP `GET` request to the `/token/list` path.
    pub async fn token_list(
        &self,
        sort_by: Option<SortBy>,
        descending: bool,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<List<TokenListInfo>> {
        self.get(
            "token/list",
            &[
                ("sortBy", sort_by.unwrap_or_default().to_string()),
                ("direction", if descending { "desc" } else { "asc" }.to_string()),
                ("limit", limit.unwrap_or(10).to_string()),
                ("offset", offset.unwrap_or(0).to_string()),
            ],
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_token_holders() {
        let client = Client::new();
        let token = "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R".parse().unwrap();
        let res = client.token_holders(&token, Some(5), None).await.unwrap();
        assert_eq!(res.data.len(), 5);
        assert!(res.total > 1000);
    }

    #[tokio::test]
    async fn test_token_meta() {
        let client = Client::new();
        let token = "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R".parse().unwrap();
        let res = client.token_meta(&token).await.unwrap();
        assert_eq!(res.address, token);
        assert_eq!(res.name, "Raydium");
        assert_eq!(res.symbol, "RAY");
    }

    #[tokio::test]
    #[ignore = "idk: missing values"]
    async fn test_token_list() {
        let client = Client::new();
        let res = client.token_list(None, true, Some(5), None).await.unwrap();
        assert_eq!(res.data.len(), 5);
        assert!(res.total > 1000);
    }
}
