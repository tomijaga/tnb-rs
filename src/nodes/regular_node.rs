use crate::account::Account;
use crate::nodes::server_node::ServerNode;
use crate::{
    models::{BlockMessage, BlockType, NodeType, PaginatedQueryTrait, TransactionQueryBuilder},
    nodes::{primary_validator::PrimaryValidator, traits::ServerNodeTrait},
    responses::{
        BlockResponse, PaginatedResponse, PrimaryValidatorConfigForNode, TransactionResponse,
    },
    utils::format_node_url,
};

use reqwest::Result;

/// A regular node that retrieves chain data and forwards blocks to the primary validator
pub struct RegularNode {
    /// base server implementation
    base: ServerNode,
}

#[allow(dead_code)]
impl ServerNodeTrait for RegularNode {
    fn get_base(&self) -> &ServerNode {
        &self.base
    }
}

#[allow(dead_code)]
impl RegularNode {
    /// Create a new instance that connects to a regular node's url
    ///
    /// ```
    ///     
    /// use tnb_rs::{nodes::RegularNode};
    ///
    /// let node_url = "https://bank.keysign.app";
    /// let node = RegularNode::new(node_url);
    ///
    /// ```
    pub fn new(url: &str) -> Self {
        RegularNode {
            base: ServerNode::new(url),
        }
    }

    /// Get transactions on the network
    ///
    /// # Inputs
    /// - **query**: Optional search parameters to sort and filter transactions
    ///
    /// # Example
    ///
    /// ```no_run
    ///     use tnb_rs::nodes::RegularNode;
    ///
    ///     let node = RegularNode::new("https://bank.keysign.app");
    ///
    ///     let response = node.get_transactions(None).unwrap();
    ///
    ///     println!("All Transactions in the network: {}", response.count);
    ///
    ///     let txs = response.results;
    ///     println!("Transactions : {:?}", txs);
    ///
    ///     assert!(txs.len() <= 50);
    ///
    /// ```
    ///
    /// # Example with a transaction query
    ///
    /// ```no_run
    ///
    ///     use tnb_rs::{
    ///         models::{NodeType, TransactionQueryBuilder, PaginatedQueryTrait},
    ///         nodes::RegularNode
    ///     };
    ///
    ///     let node = RegularNode::new("https://bank.keysign.app");
    ///
    ///     let mut tx_query = TransactionQueryBuilder::new();
    ///
    ///     let acc_to_search_for ="1329d3a5d4a5ec2382dc539e03f30c3760e01932834a23522d3de0393b63f224";
    ///     tx_query
    ///         .fee(Some(NodeType::BANK))
    ///         .limit(20)
    ///         .offset(0)
    ///         .account_number(acc_to_search_for);
    ///     
    ///     // This tx query says the node should look for the first 20 tx
    ///     // fees paid to a bank node from or to the
    ///     // given account number
    ///     
    ///     let response = node.get_transactions(Some(&tx_query)).unwrap();
    ///     println!("The total number of txs after being filtered by the query: {}", response.count);
    ///
    ///     let txs = response.results;
    ///     assert!(txs.len() <= 20);
    ///
    /// ```
    ///
    pub fn get_transactions<'a>(
        &'a self,
        query: Option<&'a TransactionQueryBuilder<'a>>,
    ) -> Result<PaginatedResponse<TransactionResponse>> {
        self.base.get_data(
            "/bank_transactions",
            match query {
                Some(q) => Some(q.get_params()),
                _ => None,
            },
        )
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

    let mut tx_query = TransactionQueryBuilder::new();

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

    let mut tx_query = TransactionQueryBuilder::new();
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
