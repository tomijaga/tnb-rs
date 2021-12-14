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
//!     println!("account number: {}", acc.account_number());
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
//!    assert_eq!(priv_key, acc.signing_key());
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
//!     let mnemonic = "visa nephew like this amazing soldier negative front elevator warfare teach good";
//!     let hd = HDWallet::from_mnemonic(mnemonic, None).unwrap();
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
//!     let result = Account::verify_signature(&sig, message, &acc.account_number());
//!     
//!     // The result will only be true if it is validated with the original message and the signer's account_number
//!     assert_eq!(result, true);
//!     
//! ```
//!
//! # Wallet
//! - Sending transactions
//!
//! ```no_run
//!
//!     use tnb_rs::{Wallet, HDWallet, Account, models::Transaction};
//!     
//!     let mnemonic = "visa nephew like this amazing soldier negative front elevator warfare teach good";
//!     let hd = HDWallet::from_mnemonic(mnemonic, None).unwrap();
//!     let acc: Account =  hd.get_first_account();
//!
//!     let bank_url = "https://bank.keysign.app";
//!     let mut wallet = Wallet::new(&acc, bank_url);
//!     
//!     // This method retrieves the nodes transaction fee details
//!     // It is important to call this method before sending a transaction
//!     wallet.init();
//!
//!     let recipient = "1329d3a5d4a5ec2382dc539e03f30c3760e01932834a23522d3de0393b63f224";
//!     let tx = Transaction::new(recipient, 1000);
//!
//!     wallet.send_transaction(&tx);
//!     
//!     // Sending Multiple transactions
//!     let recipient2 = "57d7a6e732b6280e967666a76a827bf75a0a34ace8ccbc530422c81f8d7b1239";
//!     let tx2 = Transaction::new_with_memo(recipient2, 100, "Account Withdrawal");
//!
//!     let txs = vec![tx, tx2];
//!     wallet.send_transactions(&txs);
//!
//! ```
//!
//! # Nodes
//! - Connect directly to the network nodes
//! ```
//!     use tnb_rs::nodes::{RegularNode, PrimaryValidator, ConfirmationValidator, ValidatorTrait, ServerNodeTrait};
//!     
//!     let cv = ConfirmationValidator::new("http://54.241.48.170");
//!
//!     let node = RegularNode::new("https://bank.keysign.app");
//!
//!     let pv = PrimaryValidator::new("http://52.52.160.149");
//!
//!     // or get the selected pv of a node
//!     let pv: PrimaryValidator = node.get_pv().unwrap();
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
mod client;
mod hd_wallet;
mod utils;
mod wallet;

/// Data Types for making on-chain requests
pub mod models;

/// Different nodes on the network
pub mod nodes;

/// Module with the response of every node's endpoints
pub mod responses;

pub use crate::account::Account;
pub use crate::hd_wallet::{HDWallet, MAX_CHILD_INDEX};
// pub use models::*;
pub use wallet::*;
// pub use responses::*;
