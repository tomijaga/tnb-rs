use crate::{account::Account, models::Transaction};
use regex::Regex;
use serde::Serialize;

/// Contains the structure of supported block types
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum BlockType<'a> {
    /// The Coin Transfer Block Type
    CoinTransfer {
        /// balance key of the sender's account
        balance_key: String,

        /// An array of transactions to send to the network
        txs: Vec<&'a Transaction<'a>>,
    },
}

impl BlockType<'_> {
    /// Create a new Coin Transfer BlockType
    /// - This method sorts and formats the transactions so that they can be broadcasted on the network

    pub fn coin_transfer<'a>(
        balance_lock: String,
        mut txs: Vec<&'a Transaction<'a>>,
    ) -> BlockType<'a> {
        txs.sort_by(|a, b| a.recipient.cmp(b.recipient));

        let re = Regex::new(r"^[a-zA-Z0-9_ ]*$").unwrap();

        for tx in txs.iter() {
            if tx.memo.is_some() {
                if !re.is_match(tx.memo.unwrap()) {
                    panic!("Memo can only contain alphanumeric values (Aa - Zz, 0 - 9), space and an underscore (_)");
                }
            }
        }

        BlockType::CoinTransfer {
            balance_key: balance_lock,
            txs: txs,
        }
    }
}

#[test]
fn test_regex() {
    let re = Regex::new(r"^[a-zA-Z0-9_ ]*$").unwrap();
    let valid_memo = "Testing Regex";
    let invalid_memo = "!@#$%^&*(";

    assert_eq!(re.is_match(valid_memo), true);
    assert_eq!(re.is_match(invalid_memo), false);
    assert_eq!(re.is_match("Testing Regex!"), false);
}

#[test]
fn coin_transfer_block_with_valid_memo() {
    let rand_acc = Account::new();
    let balance_lock = rand_acc.account_number().to_string();

    let transaction = Transaction {
        recipient: rand_acc.account_number(),
        fee: None,
        amount: 1,
        memo: Some("_Testing Rust library"),
    };

    BlockType::coin_transfer(balance_lock, vec![&transaction]);
}

#[test]
#[should_panic]
fn coin_transfer_block_with_invalid_memo() {
    let rand_acc = Account::new();
    let balance_lock = rand_acc.account_number().to_string();

    let transaction = Transaction {
        recipient: rand_acc.account_number(),
        fee: None,
        amount: 1,
        memo: Some("_Testing Rust library!"),
    };

    BlockType::coin_transfer(balance_lock, vec![&transaction]);
}

/// Block structure to make a block request on the network
#[derive(Debug, Serialize)]
pub struct BlockMessage<'a> {
    /// sender's account number
    pub account_number: &'a str,

    /// block message that contains the sender's request
    pub message: &'a BlockType<'a>,

    /// the signed message
    pub signature: String,
}
