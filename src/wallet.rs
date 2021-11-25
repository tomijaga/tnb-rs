use crate::{
    account::Account,
    client::Client,
    models::{Transaction, TransactionSearchParamsBuilder},
    nodes::{RegularNode, ServerNodeTrait},
    responses::BlockResponse,
};

use anyhow::Result as AnyResult;
use reqwest::Result;

/// Wallet Client for sending transactions on the network
pub struct Wallet<'a> {
    client: Client,
    account: &'a Account,
}

#[allow(dead_code)]
impl Wallet<'_> {
    /// Create a new Wallet
    ///
    /// # Example
    /// ```
    ///     use tnb_rs::{Account, Wallet};
    ///
    ///     let sk = "4b3e69add153435a30c03f6ba4576cedeacfd9d362272a39863f0f3e37eda72c";
    ///     let acc = Account::from_signing_key(sk).unwrap();
    ///
    ///     let bank_url = "https://bank.keysign.app";
    ///
    ///     let mut wallet = Wallet::new(&acc, bank_url);
    ///
    ///     wallet.init();
    ///
    /// ```
    ///
    pub fn new<'a>(account: &'a Account, node_url: &'a str) -> Wallet<'a> {
        Wallet {
            client: Client::new(node_url),
            account: account,
        }
    }

    /// Retrieve/Update the node's config files so you can send transactions
    pub fn init(&mut self) {
        self.client.update_config();
    }

    /// Send a single transaction
    ///
    /// # Example
    /// ```no_run
    ///     use tnb_rs::{Account, Wallet, models::Transaction};
    ///
    ///     let sk = "4b3e69add153435a30c03f6ba4576cedeacfd9d362272a39863f0f3e37eda72c";
    ///     let acc = Account::from_signing_key(sk).unwrap();
    ///
    ///     let bank_url = "https://bank.keysign.app";
    ///
    ///     let mut wallet = Wallet::new(&acc, bank_url);
    ///
    ///     wallet.init();
    ///
    ///     let recipient = "1329d3a5d4a5ec2382dc539e03f30c3760e01932834a23522d3de0393b63f224";
    ///     let tx = Transaction::new(recipient, 1000);
    ///
    ///     wallet.send_transaction(&tx);
    ///
    /// ```
    ///
    pub fn send_transaction<'a>(&'a self, tx: &'a Transaction) -> Result<BlockResponse> {
        let txs = vec![(*tx).clone()];

        self.client.send_transactions(self.account, &txs)
    }

    /// Send multiple transactions
    /// - This method can only send 10 transactions at once
    /// - You can't have more than one transaction with the same recipient address
    ///
    /// # Example
    /// ```no_run
    ///     use tnb_rs::{Account, Wallet, models::Transaction};
    ///
    ///     let sk = "4b3e69add153435a30c03f6ba4576cedeacfd9d362272a39863f0f3e37eda72c";
    ///     let acc = Account::from_signing_key(sk).unwrap();
    ///
    ///     let bank_url = "https://bank.keysign.app";
    ///
    ///     let mut wallet = Wallet::new(&acc, bank_url);
    ///
    ///     wallet.init();
    ///
    ///     let recipient1 = "1329d3a5d4a5ec2382dc539e03f30c3760e01932834a23522d3de0393b63f224";
    ///     let recipient2 = Account::new();
    ///     let recipient3 = Account::new();
    ///
    ///     let tx1 = Transaction::new(recipient1, 1000);
    ///     let tx2 = Transaction::new(recipient2.account_number(), 1000);
    ///     let tx3 = Transaction::new_with_memo(recipient3.account_number(), 1000, "Testing tnb_rs");
    ///
    ///     let txs = vec![tx1, tx2, tx3];
    ///
    ///     wallet.send_transactions(&txs);
    ///
    /// ```
    ///
    pub fn send_transactions<'a>(&'a self, txs: &'a Vec<Transaction>) -> Result<BlockResponse> {
        self.client.send_transactions(self.account, txs)
    }

    /// Get the number of coins held in an account
    pub fn get_balance(&self) -> AnyResult<Option<u64>> {
        self.client
            .get_account_balance(&self.account.account_number())
    }

    /// Switch to a different node to process transaction
    pub fn switch_node<'a>(&'a mut self, node_url: &'a str) -> Result<()> {
        self.client.node = RegularNode::new(node_url);
        self.init();
        Ok(())
    }
}

#[test]
fn get_balance() {
    let sk = "4b3e69add153435a30c03f6ba4576cedeacfd9d362272a39863f0f3e37eda72c";
    let acc = Account::from_signing_key(sk).unwrap();
    let node_url = "http://bank.tnbexplorer.com";

    let mut wallet = Wallet::new(&acc, node_url);
    wallet.init();

    let balance = wallet.get_balance().unwrap();

    println!("balance: {:?}", balance);
}
