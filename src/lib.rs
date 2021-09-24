//! Bespoke protocol and high-level implementation of Non-fungible token (NFT) technology 🚀
//!
//! # Licensing
//!
//! This project is duel-licensed under both the [MIT](https://en.wikipedia.org/wiki/MIT_License) and [Apache](https://en.wikipedia.org/wiki/Apache_License) licenses, so feel free to use either at your discretion.

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod block;
mod error;
mod hash;

pub use block::Block;
pub use error::{Error, Result, SignerError, VerifierError};
pub use hash::Hash;
