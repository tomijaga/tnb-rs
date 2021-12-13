use crate::{
    account::Account,
    models::{BlockType, NodeType, Transaction},
    nodes::{ServerNodeTrait, ValidatorTrait},
    responses::{
        AccountBalanceLockResponse, AccountBalanceResponse, BlockResponse, ConfigResponse,
    },
};

use crate::nodes::{PrimaryValidator, RegularNode};

use anyhow::{bail, Result as AnyResult};
use reqwest::Result;

pub struct Client {
    pub node: RegularNode,
    pub primary_validator: Option<PrimaryValidator>,
    pub node_config: Option<ConfigResponse>,
    pub primary_validator_config: Option<ConfigResponse>,
}

#[allow(dead_code)]
impl Client {
    /// Create a new Client
    pub fn new<'a>(node_url: &'a str) -> Client {
        let node = RegularNode::new(node_url);
        Client {
            node: node,
            primary_validator: None,
            node_config: None,
            primary_validator_config: None,
        }
    }

    /// Retrieve the latest config details of your node
    fn update_node_config(&mut self) {
        self.node_config = Some(self.node.get_config().unwrap());
    }

    /// Retrieve the latest config details of your node's selected primary validator
    fn update_primary_validator_config(&mut self) {
        if self.primary_validator.is_none() {
            self.primary_validator = Some(self.node.get_pv().unwrap());
        }
        let pv = self.primary_validator.as_ref().unwrap();
        self.primary_validator_config = Some(pv.get_config().unwrap());
    }

    /// Rerieve the latest config details for your node and it's selected primary validator
    pub fn update_config(&mut self) {
        self.update_primary_validator_config();
        self.update_node_config();
    }

    /// Broadcasts block from your account to the chain
    fn broadcast_block<'a>(
        &'a self,
        block_data: BlockType,
        sender: &'a Account,
    ) -> Result<BlockResponse> {
        self.node.add_blocks(&block_data, sender)
    }

    /// Retrieves the fees for the regular node and primary validator
    fn get_network_fees<'a>(&self) -> (Transaction, Transaction) {
        if self.primary_validator.is_some() {
            let pv_config = self.primary_validator_config.as_ref().unwrap();

            let pv_fee = Transaction {
                recipient: &pv_config.account_number,
                amount: pv_config.default_transaction_fee,
                fee: Some(NodeType::PRIMARY_VALIDATOR),
                memo: None,
            };

            let node_config = self.node_config.as_ref().unwrap();
            let node_fee = Transaction {
                recipient: &node_config.account_number,
                amount: node_config.default_transaction_fee,
                fee: Some(NodeType::BANK),
                memo: None,
            };

            (pv_fee, node_fee)
        } else {
            panic!("Failed to retrive Primary Validator. Try running '.init()' on the client struct before calling other methods")
        }
    }

    /// Retrieves the unique id for an account's next transaction
    fn get_account_balance_lock<'a>(&self, account: &'a Account) -> AnyResult<Option<String>> {
        if self.primary_validator.is_some() {
            let pv = self.primary_validator.as_ref().unwrap();
            let AccountBalanceLockResponse { balance_lock } = pv
                .get_account_balance_lock(account.account_number())
                .unwrap();

            Ok(balance_lock)
        } else {
            bail!("Failed to retrive Primary Validator. Try running '.init()' on the client struct before calling other methods")
        }
    }

    /// Retrieves the number of coins held by an account
    pub fn get_account_balance<'a>(&'a self, account_number: &'a str) -> AnyResult<Option<u64>> {
        let pv = self.primary_validator.as_ref().unwrap();
        let pv_response = pv.get_account_balance(account_number);

        if pv_response.is_ok() {
            let response = pv_response.unwrap();
            let AccountBalanceResponse { balance } = response;
            Ok(balance)
        } else {
            bail!("Failed to retrieve the balance key for {}", account_number);
        }
    }

    /// Send multiple transactions
    pub fn send_transactions<'a>(
        &self,
        sender: &'a Account,
        txs: &'a Vec<Transaction>,
    ) -> Result<BlockResponse> {
        let (pv_fee, node_fee) = self.get_network_fees();

        let balance_lock = self.get_account_balance_lock(sender).unwrap();

        if balance_lock.is_none() {
            panic!("This Account has never received any coins. Try sending coins to it before making a transaction");
        }

        let balance_lock = balance_lock.unwrap();

        let mut merged_txs = Vec::new();

        merged_txs.extend(txs);
        merged_txs.push(&node_fee);
        merged_txs.push(&pv_fee);

        let transfer_block = BlockType::coin_transfer(balance_lock, merged_txs);

        self.broadcast_block(transfer_block, sender)
    }

    /// Send a single transaction
    pub fn send_transaction<'a>(
        &self,
        sender: &'a Account,
        tx: &'a Transaction,
    ) -> Result<BlockResponse> {
        let txs = vec![(*tx).clone()];
        self.send_transactions(sender, &txs)
    }
}

#[test]
fn get_network_fees() {
    let url = "https://bank.keysign.app";
    let mut client = Client::new(url);
    client.update_config();

    let (pv_fee, node_fee) = client.get_network_fees();

    let node = RegularNode::new(url);
    let pv = node.get_pv().unwrap();

    let pv_config = pv.get_config().unwrap();

    assert_eq!(pv_fee.amount, pv_config.default_transaction_fee);
    assert_eq!(pv_fee.fee, Some(NodeType::PRIMARY_VALIDATOR));
    assert_eq!(pv_fee.memo, None);
    assert_eq!(pv_fee.recipient, pv_config.account_number);

    let node_config = node.get_config().unwrap();
    assert_eq!(node_fee.amount, node_config.default_transaction_fee);
    assert_eq!(node_fee.fee, Some(NodeType::BANK));
    assert_eq!(node_fee.memo, None);
    assert_eq!(node_fee.recipient, node_config.account_number);
}
