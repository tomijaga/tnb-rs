use crate::account::Account;
use crate::nodes::server_node::ServerNode;
use crate::{
    models::{
        BlockMessage, BlockType, NodeType, PaginatedSearchParams, TransactionSearchParamsBuilder,
    },
    nodes::{primary_validator::PrimaryValidator, traits::ServerNodeTrait},
    responses::{
        BlockResponse, ConfigResponse, PaginatedResponse, PrimaryValidatorConfigForNode,
        TransactionResponse,
    },
    utils::format_node_url,
};

// use anyhow::ensure;
use reqwest::Result;
use url::Url;
// use serde::Deserialize;
// use url::Host;

/// A regular node that retrieves chain data and forwards blocks to the primary validator
pub struct RegularNode {
    /// base server implementation
    pub base: ServerNode,
}

#[allow(dead_code)]
impl ServerNodeTrait for RegularNode {
    fn get_base(&self) -> &ServerNode {
        &self.base
    }
}

#[allow(dead_code)]
impl RegularNode {
    /// Create a struct that connects to a regular node's url
    pub fn new(url: &str) -> Self {
        RegularNode {
            base: ServerNode::new(url),
        }
    }
    /// Retrieve this banks primary validator
    pub fn get_pv(&self) -> Result<PrimaryValidator> {
        let config = self.get_config().unwrap();
        let PrimaryValidatorConfigForNode {
            protocol,
            ip_address,
            port,
            ..
        } = config.primary_validator.unwrap();

        let pv_url = format_node_url(&protocol, &ip_address, port);
        Ok(PrimaryValidator::new(&pv_url))
    }

    /// add blocks to this banks node then broadcast it to the network
    pub fn add_blocks<'a>(
        &'a self,
        block_data: &'a BlockType,
        account: &'a Account,
    ) -> Result<BlockResponse> {
        let block_message = account.create_block_message(block_data);
        self.base
            .post_data::<BlockMessage, BlockResponse>("/blocks", &block_message)
    }

    /// Get transactions on the network
    ///
    /// # Inputs
    /// - **query**: Optional search parameters to sort and filter transactions
    ///
    /// # Example
    ///
    /// ```
    ///     use tnb_rs::RegularNode;
    ///
    ///     let node = RegularNode::new("https://bank.keysign.app");
    ///
    ///     let response = node.get_transactions(None).unwrap();
    ///
    ///     println!("All Transactions in the network: {}", response.count);
    ///
    ///     let txs = response.results;
    ///     println!("Transactions received: {}", txs.len())
    /// ```
    pub fn get_transactions<'a>(
        &'a self,
        query: Option<&'a TransactionSearchParamsBuilder<'a>>,
    ) -> Result<PaginatedResponse<TransactionResponse>> {
        self.base.get_data(
            "/bank_transactions",
            match query {
                Some(q) => Some(q.get_params()),
                _ => None,
            },
        )
    }
}

#[test]
fn get_transactions() {
    let node = RegularNode::new("https://bank.keysign.app");

    let response = node.get_transactions(None).unwrap();
    let txs = response.results;

    println!("Total Transactions: {:?}", response.count);
    println!("Next Response Link: {:?}", response.next);

    println!("Previous Response Link: {:?}", response.previous);

    println!("Transactions Received: {:?}", txs.len());
}

fn get_offset(url: String) -> u8 {
    url.split("offset=").collect::<Vec<&str>>()[1]
        .parse::<u8>()
        .ok()
        .unwrap()
}

#[test]
fn test_null_prev_link() {
    let node = RegularNode::new("https://bank.keysign.app");

    let response = node.get_transactions(None).unwrap();

    assert!(response.prev().is_err());
}

#[test]
fn test_next_paginated_data() {
    let node = RegularNode::new("https://bank.keysign.app");

    let mut tx_query = TransactionSearchParamsBuilder::new();

    tx_query.limit(5).offset(100);

    let response = node.get_transactions(Some(&tx_query)).unwrap();
    let total = response.count;
    let len = response.results.len().clone();

    let next_response = response.next().unwrap();

    let next_next_offset = get_offset(next_response.next.clone().unwrap());

    assert_eq!(100 + 5 * 2, next_next_offset);
    assert_eq!(total, next_response.count);

    assert_eq!(len, next_response.results.len());
}

#[test]
fn get_bank_fees() {
    let node = RegularNode::new("https://bank.keysign.app");

    let mut tx_query = TransactionSearchParamsBuilder::new();
    tx_query.fee(Some(NodeType::BANK));

    let response = node.get_transactions(Some(&tx_query)).unwrap();
    let txs = response.results;

    println!("Total Transactions: {:?}", response.count);
    println!("Next Response Link: {:?}", response.next);

    println!("Previous Response Link: {:?}", response.previous);

    println!("Transactions Received: {:?}", txs.len());

    for tx in txs {
        assert_eq!(tx.fee, Some("BANK".to_string()));
    }
}
