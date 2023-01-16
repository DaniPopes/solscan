//! Transaction section.

use crate::{
    concat_1,
    solana::{Hash, Pubkey, Signature},
    Client, Result,
};
use serde_json::Value;

// TODO: fix some values

api_models! {
    pub struct TransactionMeta {
        pub err: Option<Value>,
        pub fee: Option<u64>,
        pub inner_instructions: Vec<Value>,
        pub log_messages: Vec<String>,
        pub post_balances: Vec<u64>,
        pub post_token_balances: Vec<u64>,
        pub pre_balances: Vec<u64>,
        pub pre_token_balances: Vec<u64>,
        pub rewards: Option<Value>,
        pub status: Option<Value>,
    }

    pub struct TransactionInfo {
        pub meta: TransactionMeta,
        pub transaction: Transaction,
        pub version: String,
    }

    pub struct Transaction {
        pub message: TransactionMessage,
        #[serde(with = "crate::serde_string::vec")]
        pub signatures: Vec<Signature>,
    }

    pub struct TransactionMessage {
        pub account_keys: Vec<AccountKey>,
        pub address_table_lookups: Option<Value>,
        pub instructions: Vec<Value>,
        pub recent_blockhash: Hash,
    }

    pub struct AccountKey {
        #[serde(with = "crate::serde_string")]
        pub pubkey: Pubkey,
        pub signer: bool,
        pub source: String,
        pub writable: bool,
    }

    pub struct Transaction2 {
        pub block_time: u64,
        pub slot: u64,
        #[serde(with = "crate::serde_string")]
        pub tx_hash: Hash,
        pub fee: u64,
        pub status: String,
        #[serde(alias = "signer", with = "crate::serde_string::vec")]
        pub signers: Vec<Pubkey>,
        #[serde(alias = "logMessage")]
        pub log_messages: Vec<String>,
        #[serde(alias = "inputAccount")]
        pub input_accounts: Vec<InputAccount>,
        #[serde(with = "crate::serde_string")]
        pub recent_blockhash: Hash,
        pub confirmations: Option<u64>,
        #[serde(alias = "innerInstruction")]
        pub inner_instructions: Vec<Value>,
        #[serde(alias = "token_balanes")] // yes typo
        pub token_balances: Vec<Value>,
        #[serde(alias = "parsedInstruction")]
        pub parsed_instructions: Vec<Value>,
        pub token_transfers: Vec<Value>,
        pub sol_transfers: Vec<Value>,
        pub serum_transactions: Vec<Value>,
        pub raydium_transactions: Vec<Value>,
        pub unknown_transfers: Vec<Value>,
    }

    pub struct InputAccount {
        #[serde(with = "crate::serde_string")]
        pub account: Pubkey,
        pub signer: bool,
        pub writable: bool,
        pub pre_balance: u64,
        pub post_balance: u64,
    }
}

impl Client {
    /// Performs an HTTP `GET` request to the `/transaction/last` path.
    pub async fn transaction_last(&self, limit: Option<u64>) -> Result<Vec<TransactionInfo>> {
        self.get("transaction/last", &[("limit", limit.unwrap_or(20))]).await
    }

    /// Performs an HTTP `GET` request to the `/transaction/{signature}` path.
    pub async fn transaction(&self, signature: &Signature) -> Result<Transaction2> {
        self.get_no_query(dbg!(&concat_1("transaction/", &signature.to_string()))).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "unreliable: empty"]
    async fn test_transaction_last() {
        let client = Client::new();
        let res = client.transaction_last(Some(5)).await.unwrap();
        assert_eq!(res.len(), 5);
    }

    #[tokio::test]
    #[ignore = "unreliable: 404"]
    async fn test_transaction() {
        let client = Client::new();
        let s = "5YdxDGc9Ki1iAPfNwX4JjGShXUQ7YMd85zEygZdVhk1p8WtnfEdGyJ9cnVVuYLULYrVD6ogdHsy3eNdL9viM5hS6";
        let res = client.transaction(&s.parse().unwrap()).await.unwrap();
        assert_eq!(res.tx_hash, s.parse::<Hash>().unwrap());
    }
}