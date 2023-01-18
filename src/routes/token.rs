//! Token section.

use crate::{solana::Pubkey, Client, Result};
use std::{collections::HashMap, fmt};

api_models! {
    pub struct TokenList<T> {
        /// The requested items.
        pub data: Vec<T>,
        /// Not the length of the request list, but the amount of all items.
        pub total: u64,
    }

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

    pub struct TokenListInfo {
        pub price_ust: f64,
        #[serde(default)]
        pub tag: Option<Vec<String>>,
        pub token_name: String,
        pub token_symbol: String,
        pub twitter: String,
        pub website: String,
        pub coingecko_info: CoingeckoInfo,
        pub sol_alpha_volume: Option<f64>,
        #[serde(rename = "_id")]
        pub id: String,
        #[serde(with = "crate::serde_string")]
        pub address: Pubkey,
        pub created_at: String,
        pub decimals: u64,
        /// Any metadata extension. Should always contain `"coingeckoId"`.
        pub extensions: HashMap<String, String>,
        pub icon: String,
        pub is_violate: bool,
        pub market_cap_rank: u64,
        pub mint_address: String,
        pub symbol_has_lower: bool,
        pub updated_at: String,
        pub holder: u64,
        #[serde(rename = "marketCapFD")]
        pub market_cap_fd: Option<f64>,
    }

    pub struct CoingeckoInfo {
        pub coingecko_rank: Option<u64>,
        pub market_cap_rank: Option<u64>,
        pub market_data: CoingeckoMarketData,
    }

    pub struct CoingeckoMarketData {
        pub current_price: f64,
        pub ath: f64,
        pub ath_change_percentage: f64,
        pub ath_date: String,
        pub atl: f64,
        pub atl_change_percentage: f64,
        pub atl_date: String,
        pub market_cap: f64,
        pub market_cap_rank: Option<u64>,
        pub fully_diluted_valuation: Option<f64>,
        pub total_volume: Option<f64>,
        pub price_high24h: Option<f64>,
        pub price_low24h: Option<f64>,
        pub price_change24h: Option<f64>,
        pub price_change_percentage24h: Option<f64>,
        pub price_change_percentage7d: Option<f64>,
        pub price_change_percentage14d: Option<f64>,
        pub price_change_percentage30d: Option<f64>,
        pub price_change_percentage60d: Option<f64>,
        pub price_change_percentage200d: Option<f64>,
        pub price_change_percentage1y: Option<f64>,
        pub market_cap_change24h: Option<f64>,
        pub market_cap_change_percentage24h: Option<f64>,
        pub total_supply: Option<f64>,
        pub max_supply: Option<f64>,
        pub circulating_supply: Option<f64>,
        pub last_updated: String,
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

impl From<SortBy> for &'static str {
    fn from(val: SortBy) -> Self {
        use SortBy::*;
        match val {
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
    ) -> Result<TokenList<TokenHolderData>> {
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
    ) -> Result<TokenList<TokenListInfo>> {
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
    use crate::solana::Pubkey;

    static TOKEN: &str = "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R";

    test_route!(test_token_holders: |c| c.token_holders(&TOKEN.parse().unwrap(), Some(5), None) => |res| {
        assert_eq!(res.data.len(), 5);
        assert!(res.total > 1000);
    });

    test_route!(test_token_meta: |c| c.token_meta(&TOKEN.parse().unwrap()) => |res| {
        assert_eq!(res.address, TOKEN.parse::<Pubkey>().unwrap());
        assert_eq!(res.name, "Raydium");
        assert_eq!(res.symbol, "RAY");
    });

    test_route!(test_token_list: |c| c.token_list(None, true, Some(100), None) => |res| {
        assert_eq!(res.data.len(), 100);
        assert!(res.total > 1000);
    });
}
