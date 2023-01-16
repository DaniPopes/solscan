#[cfg(any(feature = "sdk", feature = "sdk-full"))]
mod imp {
    pub use solana_sdk::{account::Account, hash::Hash, pubkey::Pubkey};

    #[cfg(feature = "sdk-full")]
    pub use solana_sdk::signature::Signature;

    #[cfg(not(feature = "sdk-full"))]
    pub type Signature = String;
}

#[cfg(not(any(feature = "sdk", feature = "sdk-full")))]
#[allow(dead_code)]
mod imp {
    pub type Account = String;
    pub type Hash = String;
    pub type Signature = String;
    pub type Pubkey = String;
}

pub use self::imp::*;
