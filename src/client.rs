use crate::account::*;
use crate::node::common::ClientNode;
use crate::node::server_node::BlockRecord;
use crate::{Bank, ConfigResponse, PrimaryValidator, PrimaryValidatorConfig};

use reqwest::Result;

pub struct Client<'a> {
    node: Bank,
    primary_validator: Option<PrimaryValidator>,
    account: &'a Account,
    node_config: Option<ConfigResponse>,
    primary_validator_config: Option<ConfigResponse>,
}

pub enum AccountOrAddress {
    Account,
    String,
}

impl Client<'_> {
    pub fn new<'a>(account: &'a Account, node_url: &'a str) -> Client<'a> {
        let node = Bank::new(node_url);
        Client {
            account: account,
            node: node,
            primary_validator: None,
            primary_validator_config: None,
            node_config: None,
        }
    }

    fn update_node_config(&mut self) {
        self.node_config = Some(self.node.get_config().unwrap());
    }

    fn update_primary_validator_config(&mut self) {
        if let Some(pv) = &self.primary_validator {
            self.primary_validator_config = Some(pv.get_config().unwrap());
        }
    }

    pub fn update_config(&mut self) {
        if self.primary_validator.is_none() {
            self.primary_validator = Some(self.node.get_pv().unwrap());
        }
        self.update_primary_validator_config();
        self.update_node_config();
    }

    fn broadcast_transaction<'a>(
        &'a self,
        balance_lock: &'a str,
        txs: Vec<&'a Transaction>,
        sender: &'a Account,
    ) -> Result<BlockRecord> {
        let block_data = BlockData::coin_transfer(balance_lock, txs);
        self.node.add_blocks(&block_data, sender)
    }

    // fn create_network_transaction(&self) {

    //     self.
    // }

    // fn send_transaction(&self, tx: &Transaction) -> Result<BlockRecord> {
    //     self.broadcast_transaction(
    //         balance_lock: &'a str,
    //         txs: Vec<&'a Transaction>,
    //         account: &'a Account,
    //     )
    // }
}
