//!
//! This crate is an SDk for [thenewboston](https://thenewboston.com/) cryptocurrency network and
//! it supports account generation, signing and signature verification
//!
//!
//! # Account
//! Accounts are annonymous identites with a private_key and a public_key called an account_number where coins can be sent to and from.
//! The person who has access to an accounts private key has total control over the accounts coins so ensure you store your private key safely
//!
//! ## Create a Random Account
//!
//! ```
//! use tnb_rs::account::Account;
//!
//! let acc = Account::new()
//! ```
//!
//! ## Create an account from an existing private_key
//!
//! ```
//!    let priv_key = "8cf08eb96b00b5a4df86a750bb7ae595a9dbbe91fc091463bfb3d950d5dac467"
//!    let acc = Account::from_private_key(priv_key);
//!
//!     assert_eq!(priv_key, acc.private_key_hex());
//! ```
//!
//! ## Signature
//!
//! - Creating a signature from a message
//!
//! ```
//!     use tnb_rs::account::Account;
//!     let priv_key = "8cf08eb96b00b5a4df86a750bb7ae595a9dbbe91fc091463bfb3d950d5dac467"
//!     let acc = Account::from_private_key(priv_key);
//!
//!     let message = "Hidden Message";
//!
//!     let sig = acc.create_signature(message);
//!
//!     println!("signature: {}", sig);
//!     
//! ```
//!
//! - Validating a signature
//!
//!    The result will only be true if it is validated with the original message and the signer's account_number
//!
//! ```
//!     let result = Account::validate_signature(&sig, message, acc.account_number);
//!
//!     println!("result: {}", result);
//!
//!     
//! ```
//!
//! # HDWallet
//! - For the power users who may want to utilize a lot of accounts
//! - A mnemonic phrase that is easier to remember than a 32 byte hex number
//! - Useful for staying hidden on a public and decentralized network such as thenewboston
//!
//! ## Create a HDWallet
//!  - Creating a random wallet
//!     
//!     
//!     ```
//!         let hd = HDWallet::new();
//!         
//!         // remember to save the mnemonic phrase before proceeding
//!         // if you plan on using this hd wallet again
//!         println!("mnemonic phrase: {}", hd.mnemonic);
//!     ```
//!     
//!  - Creating a hd wallet from a mnemonic phrase
//!
//!     ```
//!         let m = "visa nephew like this amazing soldier negative front elevator warfare teach good";
//!
//!         let hd = HDWallet::from_mnemonic_phrase(m);
//!
//!     ```
//!
//! ## Generating accounts
//!
//! - This wallet uses the slip10 and bip44 standards for Account key derivation

//!     ```
//!         
//!         let m = "visa nephew like this amazing soldier negative front elevator warfare teach good";
//!
//!         let hd = HDWallet::from_mnemonic_phrase(m);
//!         
//!         // simple method to get the first account in the hd waller tree
//!         // Useful for people who want to use the hd wallet to memorizing an account
//!         let first_account = hd.first_account();
//!         
//!
//!     ```
//!     
//! - Generating a specific accounts from the hd wallet tree
//! - This is done by calling the `get_account` method with an account and address index
//!
//!     ```
//!         let acc =  hd.get_account(123, 78);
//!
//!     ```
//!     
//!     
//!
//!
pub mod account;
pub mod hd_wallet;

pub use account::{Account, BlockData, ChainData, Transaction};
pub use hd_wallet::HDWallet;
