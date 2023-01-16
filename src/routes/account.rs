//! Account section.

use crate::{concat_1, Client, Result};
use solana_sdk::pubkey::Pubkey;

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
}

impl From<Account> for solana_sdk::account::Account {
    fn from(value: Account) -> Self {
        let Account { lamports, owner_program, rent_epoch, .. } = value;
        Self { lamports, data: vec![], owner: owner_program, executable: false, rent_epoch }
    }
}

impl Client {
    /// Performs an HTTP `GET` request to the `/account/{account}` path.
    pub async fn account(&self, account: &Pubkey) -> Result<Account> {
        self.get(&concat_1("account/", &account.to_string()), &()).await
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_account() {
        let client = Client::new();
        let account = "9doNJz52PMd8bi3FKRV9gfXa5nXLDjoW1SBVMbmAuuSh".parse().unwrap();
        let res = client.account(&account).await.unwrap();
        assert_eq!(res.account, account);
        assert_ne!(res.lamports, 0);
    }
}
