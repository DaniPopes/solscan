//! Solscan API - Block section

use crate::{
    concat_1, Client, ClientError, ResponseError, ResponseErrorMessage, Result, TransactionInfo,
};
use serde::Deserialize;
use solana_sdk::hash::Hash;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo {
    pub block_height: Option<u64>,
    pub block_time: Option<u64>,
    #[serde(with = "crate::serde_string")]
    pub blockhash: Hash,
    pub parent_slot: u64,
    #[serde(with = "crate::serde_string")]
    pub previous_blockhash: Hash,
    pub fee_rewards: u64,
    pub transaction_count: u64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum BlockResult {
    Ok(BlockInfo),
    Err { code: i32, message: String },
}

impl From<BlockResult> for Result<BlockInfo> {
    fn from(value: BlockResult) -> Self {
        value.result()
    }
}

impl BlockResult {
    pub fn result(self) -> Result<BlockInfo> {
        match self {
            Self::Ok(x) => Ok(x),
            Self::Err { code, message } => Err(ClientError::Response(ResponseError {
                status: code,
                error: ResponseErrorMessage { message },
            })),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub current_slot: u64,
    pub result: BlockResult,
}

impl Client {
    /// Performs an HTTP `GET` request to the `/block/last` path.
    pub async fn block_last(&self, limit: Option<u64>) -> Result<Vec<Block>> {
        self.get("block/last", &[("limit", limit.unwrap_or(10))]).await
    }

    /// Performs an HTTP `GET` request to the `/block/transactions` path.
    pub async fn block_transactions(
        &self,
        block: u64,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<Vec<TransactionInfo>> {
        self.get(
            "block/transactions",
            &[("block", block), ("limit", limit.unwrap_or(10)), ("offset", offset.unwrap_or(0))],
        )
        .await
    }

    /// Performs an HTTP `GET` request to the `/block/{block}` path.
    pub async fn block(&self, block: u64) -> Result<Block> {
        self.get_no_query(&concat_1("block/", &block.to_string())).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLOT: u64 = 172407028;
    const HEIGHT: u64 = 156357404;
    const TIME: u64 = 1673681099;

    #[tokio::test]
    async fn test_block_last() {
        let client = Client::new();
        let res = client.block_last(Some(5)).await.unwrap();
        assert_eq!(res.len(), 5);
    }

    #[tokio::test]
    #[ignore = "idk: missing values"]
    async fn test_block_transactions() {
        let client = Client::new();
        let res = client.block_transactions(1, None, None).await.unwrap();
        assert!(!res.is_empty());
    }

    #[tokio::test]
    #[ignore = "block missing from storage"]
    async fn test_block() {
        let hash: Hash = "3ZQLpHEui4usw8qDNvMTaVjhFzr2upXAyCoysBdgpj52".parse().unwrap();

        let client = Client::new();
        let res = client.block(SLOT).await.unwrap();
        assert_eq!(res.current_slot, SLOT);
        let res = res.result.result().unwrap();
        assert_eq!(res.block_height, Some(HEIGHT));
        assert_eq!(res.block_time, Some(TIME));
        assert_eq!(res.blockhash, hash);
    }

    #[tokio::test]
    async fn test_block1() {
        let client = Client::new();
        let res = client.block(1).await.unwrap();
        assert_eq!(res.current_slot, 1);
        let _ = res.result.result().unwrap();
    }
}
