//! Account section.

use crate::{
    concat_1,
    solana::{Pubkey, Signature},
    Client, Result,
};

// TODO: remaining routes: stakeAccounts, splTransfers, solTransfers, exportTransactions

api_models! {
    pub struct AccountToken {
        #[serde(with = "crate::serde_string")]
        pub token_address: Pubkey,
        pub token_amount: TokenAmount,
        #[serde(with = "crate::serde_string")]
        pub token_account: Pubkey,
        pub token_name: String,
        pub token_icon: String,
        pub rent_epoch: u64,
        pub lamports: u64,
        pub token_symbol: Option<String>,
    }

    pub struct TokenAmount {
        pub amount: String,
        pub decimals: u64,
        pub ui_amount: f64,
        pub ui_amount_string: String,
    }

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

    pub struct Account {
        pub lamports: u64,
        #[serde(with = "crate::serde_string")]
        pub owner_program: Pubkey,
        pub r#type: String,
        pub rent_epoch: u64,
        #[serde(with = "crate::serde_string")]
        pub account: Pubkey,
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
    /// Performs an HTTP `GET` request to the `/account/tokens` path.
    pub async fn account_tokens(&self, account: &Pubkey) -> Result<Vec<AccountToken>> {
        self.get("account/tokens", &[("account", account.to_string())]).await
    }

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
    use crate::solana::Pubkey;

    static ACCOUNT: &str = "3SKLz31aEBqQQYeiGaezGP7v7ZEJvAmSGwBqU1zLJkgn";

    test_route!(test_account_tokens: |c| c.account_tokens(&ACCOUNT.parse().unwrap()) => |res| {
        assert!(!res.is_empty());
    });

    test_route!(test_account_transactions: |c| c.account_transactions(&ACCOUNT.parse().unwrap(), None, None) => |res| {
        assert!(!res.is_empty());
    });

    test_route!(test_account: |c| c.account(&ACCOUNT.parse().unwrap()) => |res| {
        assert_eq!(res.account, ACCOUNT.parse::<Pubkey>().unwrap());
        assert_ne!(res.lamports, 0);
    });
}
