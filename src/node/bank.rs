use crate::account::{Account, BlockData, BlockMessage, NodeType, Transaction};
use crate::node::server_node::{BlockRecord, ServerNode};
use crate::node::{
    common::{format_url, ClientNode, ConfigResponse},
    primary_validator::PrimaryValidator,
};

use anyhow::ensure;
use reqwest::Result;
use serde::Deserialize;
// use url::Host;

/// Bank
pub struct Bank {
    /// base server implementation
    pub server: ServerNode,
}

#[allow(dead_code)]
impl ClientNode for Bank {
    fn new(url: &str) -> Self {
        Bank {
            server: ServerNode::new(url, None),
        }
    }

    fn get_config(&self) -> Result<ConfigResponse> {
        self.server.get_config()
    }
}

#[allow(dead_code)]
impl Bank {
    /// Retieve this banks primary validator
    pub fn get_pv(&self) -> Result<PrimaryValidator> {
        let config = self.get_config().unwrap();
        let ConfigResponse {
            protocol,
            ip_address,
            port,
            ..
        } = config;

        let pv_url = format_url(&protocol, &ip_address, port);
        Ok(PrimaryValidator::new(&pv_url))
    }

    /// add blocks to this banks node then broadcast it to the network
    pub fn add_blocks<'a>(
        &'a self,
        block_data: &'a BlockData,
        account: &'a Account,
    ) -> Result<BlockRecord> {
        let block_message = account.create_block_message(block_data);
        self.server
            .post_data::<BlockMessage, BlockRecord>("/blocks", &block_message)
    }
}
