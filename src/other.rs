//! Solscan API - Market, Chain information and Tools sections

use super::{concat_1, Client, Result};
use serde::Deserialize;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenMarketInfo {
    pub price_usdt: f32,
    pub volume_usdt: u64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainInfo {
    pub block_height: u64,
    pub current_epoch: u64,
    pub absolute_slot: u64,
    pub transaction_count: u64,
}

impl Client {
    /// Performs an HTTP `GET` request to the `/market/token/{token}` path.
    pub async fn market(&self, token: &Pubkey) -> Result<TokenMarketInfo> {
        self.get_no_query(&concat_1("market/token/", &token.to_string())).await
    }

    /// Performs an HTTP `GET` request to the `/chaininfo` path.
    pub async fn chain_info(&self) -> Result<ChainInfo> {
        self.get_no_query("chaininfo").await
    }

    // TODO: Return value
    /// Performs an HTTP `GET` request to the `/tools/inspect` path.
    pub async fn tools_inspect(&self, message: String) -> Result<Value> {
        self.get("tools/inspect", &[("message", message)]).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_market() {
        let client = Client::new();
        let token = "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R".parse().unwrap();
        let res = client.market(&token).await.unwrap();
        assert!(res.price_usdt.is_normal());
        assert_ne!(res.volume_usdt, 0);
    }

    #[tokio::test]
    async fn test_chain_info() {
        let client = Client::new();
        let res = client.chain_info().await.unwrap();
        assert!(res.block_height > 156339814);
    }

    #[tokio::test]
    async fn test_tools_inspect() {
        let client = Client::new();
        let err = client.tools_inspect(String::new()).await.unwrap();
        let err: super::super::ResponseError = serde_json::from_value(err).unwrap();
        assert_eq!(err.status, 500);
    }
}
