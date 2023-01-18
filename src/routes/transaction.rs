//! Transaction section.

use crate::{
    concat_1,
    solana::{Hash, Pubkey, Signature},
    Client, Result,
};
use serde_json::Value;

api_models! {
    pub struct TransactionInfo {
        #[serde(default)]
        pub meta: Option<TransactionMeta>,
        pub transaction: Transaction,
        #[serde(default)]
        pub version: Option<String>,
    }

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

    pub struct Transaction {
        pub message: TransactionMessage,
        #[serde(with = "crate::serde_string::vec")]
        pub signatures: Vec<Signature>,
    }

    pub struct TransactionMessage {
        pub account_keys: Vec<TransactionAccountKey>,
        pub address_table_lookups: Option<Value>,
        pub instructions: Vec<Value>,
        #[serde(with = "crate::serde_string")]
        pub recent_blockhash: Hash,
    }

    pub struct TransactionAccountKey {
        #[serde(with = "crate::serde_string")]
        pub pubkey: Pubkey,
        pub signer: bool,
        pub source: String,
        pub writable: bool,
    }

    // So many typos
    pub struct GetTransactionInfo {
        pub block_time: u64,
        pub slot: u64,
        #[serde(with = "crate::serde_string")]
        pub tx_hash: Signature,
        pub fee: u64,
        pub status: String,
        #[serde(rename = "lamport")]
        pub lamports: u64,
        #[serde(rename = "signer", with = "crate::serde_string::vec")]
        pub signers: Vec<Pubkey>,
        #[serde(rename = "logMessage")]
        pub log_messages: Vec<String>,
        #[serde(rename = "inputAccount")]
        pub input_accounts: Vec<TransactionInputAccount>,
        #[serde(with = "crate::serde_string")]
        pub recent_blockhash: Hash,
        pub inner_instructions: Vec<Value>,
        #[serde(rename = "tokenBalanes")]
        pub token_balances: Vec<Value>,
        #[serde(rename = "parsedInstruction")]
        pub parsed_instructions: Vec<Value>,
        pub confirmations: Option<u64>,
        pub version: String,
        pub token_transfers: Vec<Value>,
        pub sol_transfers: Vec<Value>,
        pub serum_transactions: Vec<Value>,
        pub raydium_transactions: Vec<Value>,
        pub unknown_transfers: Vec<Value>,
    }

    pub struct TransactionInputAccount {
        #[serde(with = "crate::serde_string")]
        pub account: Pubkey,
        pub signer: bool,
        pub writable: bool,
        pub pre_balance: u64,
        pub post_balance: u64,
    }
}

#[cfg(feature = "sdk-full")]
impl From<TransactionInfo> for solana_sdk::transaction::Transaction {
    fn from(value: TransactionInfo) -> Self {
        value.transaction.into()
    }
}

#[cfg(feature = "sdk-full")]
impl From<Transaction> for solana_sdk::transaction::Transaction {
    fn from(value: Transaction) -> Self {
        let Transaction { message, signatures } = value;
        let TransactionMessage {
            account_keys,
            address_table_lookups: _,
            instructions: _,
            recent_blockhash,
        } = message;

        let account_keys = account_keys.into_iter().map(|key| key.pubkey).collect();
        let message =
            solana_sdk::message::Message { account_keys, recent_blockhash, ..Default::default() };

        Self { signatures, message }
    }
}

impl Client {
    /// Performs an HTTP `GET` request to the `/transaction/last` path.
    pub async fn transaction_last(&self, limit: Option<u64>) -> Result<Vec<TransactionInfo>> {
        self.get("transaction/last", &[("limit", limit.unwrap_or(20))]).await
    }

    /// Performs an HTTP `GET` request to the `/transaction/{signature}` path.
    pub async fn transaction(&self, signature: &Signature) -> Result<GetTransactionInfo> {
        self.get_no_query(&concat_1("transaction/", &signature.to_string())).await
    }
}

#[cfg(test)]
mod tests {
    test_route!(test_transaction_last: |c| c.transaction_last(Some(20)) => |res| {
        if !res.is_empty() {
            assert_eq!(res.len(), 20)
        }
    });

    test_route!(test_transaction: |c| async {
        let last_txs = c.transaction_last(Some(1)).await.unwrap();
        let sig = last_txs.first().unwrap().transaction.signatures.first().unwrap();
        c.transaction(sig).await.map(|res| {
            assert_eq!(res.tx_hash, *sig);
            res
        })
    } => |_res| {});
}
