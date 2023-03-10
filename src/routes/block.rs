//! Block section.

use crate::{
    concat_1, solana::Hash, Client, ClientError, ResponseError, ResponseErrorMessage, Result,
    TransactionInfo,
};

api_models! {
    pub struct BlockInfo {
        pub block_height: Option<u64>,
        pub block_time: Option<u64>,
        #[serde(with = "crate::serde_string")]
        pub blockhash: Hash,
        pub fee_rewards: u64,
        pub parent_slot: u64,
        #[serde(with = "crate::serde_string")]
        pub previous_blockhash: Hash,
        pub transaction_count: u64,
    }

    #[serde(untagged)]
    pub enum BlockResult {
        Ok(BlockInfo),
        Err { code: i32, message: String },
        #[default]
        Fallback
    }

    pub struct Block {
        pub current_slot: u64,
        pub result: BlockResult,
    }
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
            _ => Err(ClientError::EmptyResponse),
        }
    }
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
    test_route!(test_block_last: |c| c.block_last(Some(5)) => |res| {
        assert_eq!(res.len(), 5);
    });

    test_route!(test_block_transactions: |c| c.block_transactions(1, None, None) => |res| {
        assert!(!res.is_empty());
    });

    test_route!(test_block: |c| c.block(1) => |res| {
        assert_eq!(res.current_slot, 1);
        let _ = res.result.result().unwrap();
    });
}
