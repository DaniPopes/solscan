//! Solscan API routes.

mod block;
pub use block::*;

mod transaction;
pub use transaction::*;

mod account;
pub use account::*;

mod token;
pub use token::*;

mod other;
pub use other::*;
