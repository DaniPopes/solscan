//! Account section.

use crate::{
    concat_1,
    solana::{Pubkey, Signature},
    Client, Result,
};

// TODO: remaining routes

api_models! {
    pub struct Account {
        pub lamports: u64,
        #[serde(with = "crate::serde_string")]
        pub owner_program: Pubkey,
        pub r#type: String,
        pub rent_epoch: u64,
        #[serde(with = "crate::serde_string")]
        pub account: Pubkey,
    }

    // pub struct AccountTransaction {
    //     pub block_time: i64,
    //     pub fee: i64,
    //     pub lamport: i64,
    //     pub parsed_instruction: Vec<ParsedInstruction>,
    //     pub signer: Vec<String>,
    //     pub slot: i64,
    //     pub status: String,
    //     pub tx_hash: String,
    // }

    // pub struct ParsedInstruction {
    //     pub program_id: String,
    //     #[serde(rename = "type")]
    //     pub type_field: String,
    // }

    pub struct AccountTransaction {
        pub block_time: u64,
        pub fee: u64,
        pub lamport: u64,
        pub parsed_instruction: Vec<ParsedInstruction>,
        #[serde(with = "crate::serde_string::vec")]
        pub signer: Vec<Pubkey>,
        pub slot: u64,
        pub status: String,
        #[serde(with = "crate::serde_string")]
        pub tx_hash: Signature,
    }

    pub struct ParsedInstruction {
        #[serde(with = "crate::serde_string")]
        pub program_id: Pubkey,
        pub r#type: String,
    }
}

#[cfg(feature = "sdk")]
impl From<Account> for crate::solana::Account {
    fn from(value: Account) -> Self {
        let Account { lamports, owner_program, rent_epoch, .. } = value;
        Self { lamports, data: vec![], owner: owner_program, executable: false, rent_epoch }
    }
}

impl Client {
    /// Performs an HTTP `GET` request to the `/account/transactions` path.
    pub async fn account_transactions(
        &self,
        account: &Pubkey,
        before_hash: Option<&Signature>,
        limit: Option<u64>,
    ) -> Result<Vec<AccountTransaction>> {
        let mut query: Vec<(&str, String)> = Vec::with_capacity(3);
        query.push(("account", account.to_string()));
        if let Some(before_hash) = before_hash {
            query.push(("beforeHash", before_hash.to_string()));
        }
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }
        self.get("account/transactions", &query).await
    }

    /// Performs an HTTP `GET` request to the `/account/{account}` path.
    pub async fn account(&self, account: &Pubkey) -> Result<Account> {
        self.get_no_query(&concat_1("account/", &account.to_string())).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static ACCOUNT: &str = "3SKLz31aEBqQQYeiGaezGP7v7ZEJvAmSGwBqU1zLJkgn";

    #[tokio::test]
    async fn test_account_transactions() {
        let client = Client::new();
        let account = ACCOUNT.parse().unwrap();
        let res = client.account_transactions(&account, None, None).await.unwrap();
        assert!(!res.is_empty());
    }

    #[tokio::test]
    async fn test_account() {
        let client = Client::new();
        let account = ACCOUNT.parse().unwrap();
        let res = client.account(&account).await.unwrap();
        assert_eq!(res.account, account);
        assert_ne!(res.lamports, 0);
    }
}
