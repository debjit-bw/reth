//! The entire implementation of the namespace is quite large, hence it is divided across several
//! files.

pub mod signer;
pub mod traits;

mod block;
mod call;
mod fees;
#[cfg(feature = "optimism")]
pub mod optimism;
#[cfg(not(feature = "optimism"))]
mod pending_block;
#[cfg(not(feature = "optimism"))]
mod receipt;
mod spec;
mod state;
mod trace;
mod transaction;
