//!
//! This crate is a SDk for [thenewboston](https://thenewboston.com/) cryptocurrency network and it
//! supports account generation, signing and signature verification
//!
//!
//! # Account
//! - Accounts on [thenewboston](https://thenewboston.com/) are anonymous identites with a signing key and an account number where coins can be sent to and from.
//! - The person who has access to an account's signing key has total control over the account's coins so ensure you store your signing key safely
//!
//! ## Create a Random Account
//!
//! ```
//!     use tnb_rs::Account;
//!
//!     let acc = Account::new();
//!
//!     println!("account number: {}", acc.account_number_hex());
//! ```
//!
//! ## Create an account from an existing signing key
//!
//! ```
//!    use tnb_rs::Account;
//!
//!    let priv_key = "8cf08eb96b00b5a4df86a750bb7ae595a9dbbe91fc091463bfb3d950d5dac467";
//!    let acc = Account::from_signing_key(priv_key).unwrap();
//!
//!    assert_eq!(priv_key, acc.signing_key_hex());
//! ```
//!
//!
//! # HDWallet
//! Reasons to use a HD Wallet:
//! - Uses a mnemonic phrase that is easier to remember than a 32 byte hex number
//! - Can generate a lot of child accounts up to 4,611,686,014,132,420,600 for the bip44 standard
//! - Useful for staying hidden on a public and decentralized network such as thenewboston
//!
//! ## Create a HDWallet
//!  - Creating a random HD Wallet
//!
//! ```
//!     use tnb_rs::HDWallet;
//!
//!     let hd = HDWallet::new();
//!     
//!     // store your mnemonic phrase safely
//!     println!("mnemonic phrase: {}", hd.mnemonic());
//!
//! ```
//!
//!  - Creating a HD Wallet from a mnemonic phrase
//! > This method allows the user to set an optional password for added security on their account keys.
//!
//! ```
//!     use tnb_rs::HDWallet;
//!
//!     let m = "visa nephew like this amazing soldier negative front elevator warfare teach good";
//!
//!     // with no password
//!     let hd = HDWallet::from_mnemonic(m, None).unwrap();
//!
//!     // with password
//!     let hd = HDWallet::from_mnemonic(m, Some("********")).unwrap();
//!
//!     
//!     assert_eq!(hd.mnemonic(), m);
//!
//! ```
//!
//! # Signature
//!
//! - Creating and verifing a signature
//!
//! ```
//!     use tnb_rs::{Account, HDWallet};
//!     
//!     let m = "visa nephew like this amazing soldier negative front elevator warfare teach good";
//!     let hd = HDWallet::from_mnemonic(m, None).unwrap();
//!     
//!     let acc: Account =  hd.get_first_account();
//!     let message = "Hidden Message";
//!
//!     // Create Signature
//!     let sig = acc.create_signature(message);
//!
//!     assert_eq!(sig.len(), 128);
//!     println!("signature: {}", sig);
//!
//!
//!     // Verify Signature
//!     let result = Account::verify_signature(&sig, message, &acc.account_number_hex());
//!     
//!     // The result will only be true if it is validated with the original message and the signer's account_number
//!     assert_eq!(result, true);
//!     
//! ```
//!

#![warn(future_incompatible)]
#![deny(missing_docs)] // refuse to compile if documentation is missing
#![cfg_attr(not(test), forbid(unsafe_code))]

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

mod account;
mod hd_wallet;

pub use crate::account::{
    Account, BlockData, BlockMessage, ChainData, NodeType, SignedMessage, Transaction,
};
pub use crate::hd_wallet::{HDWallet, MAX_CHILD_INDEX};
