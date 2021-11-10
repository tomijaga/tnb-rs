use reqwest::{Client, Result};
use serde::{de, Deserialize, Serialize};
// use std::any::Any;
use url::{Origin, Url};

use crate::account::*;
use crate::node::common::ConfigResponse;

#[derive(Default)]
#[allow(dead_code)]
pub struct NodeServerOptions {
    pub limit: u32,
    pub offset: u32,
}

pub type URLQuery = (String, String);

#[derive(Clone, Debug)]
pub struct NodeServerOptionsIterator {
    pub index: usize,
    pub query: Vec<URLQuery>,
}

impl Iterator for NodeServerOptionsIterator {
    type Item = URLQuery;
    fn next<'b>(&'b mut self) -> Option<Self::Item> {
        let result = if self.query.len() > self.index {
            let (key, val) = &self.query[self.index];

            Some((key.clone(), val.clone()))
        } else {
            None
        };
        self.index += 1;
        result
    }
}

impl IntoIterator for NodeServerOptions {
    type Item = URLQuery;
    type IntoIter = NodeServerOptionsIterator;

    fn into_iter(self) -> NodeServerOptionsIterator {
        NodeServerOptionsIterator {
            index: 0,
            query: vec![
                ("limit".to_string(), self.limit.to_string()),
                ("offset".to_string(), self.offset.to_string()),
                ("format".to_string(), "json".to_string()),
            ],
        }
    }
}

/// Base Api for a node
#[allow(dead_code)]
pub struct ServerNode {
    /// url address of node's server
    pub url: String,

    /// default options for each request
    pub options: NodeServerOptions,
}

#[allow(dead_code)]
impl ServerNode {
    /// Iniatialize new Node
    pub fn new(url: &str, options: Option<NodeServerOptions>) -> Self {
        let parsed_url = Url::parse(url).unwrap();

        let origin = parsed_url.origin();

        let formatted_url = match origin {
            Origin::Tuple(protocol, host, port) => format!("{}://{}:{}", protocol, host, port),
            _ => String::new(),
        };

        ServerNode {
            url: formatted_url,
            options: match options {
                Some(server_options) => server_options,
                None => Default::default(),
            },
        }
    }

    /// Get request to the Node's Server
    #[tokio::main]
    pub async fn get_data<T: de::DeserializeOwned>(
        &self,
        endpoint: &str,
        query: Option<NodeServerOptions>,
    ) -> Result<T> {
        let url_endpoint = format!("{}{}", self.url, endpoint);
        let url = match query {
            Some(query_params) => Url::parse_with_params(&url_endpoint, query_params).unwrap(),
            None => Url::parse(&url_endpoint).unwrap(),
        };

        let response = reqwest::get(url).await?;
        Ok(response.json::<T>().await?)
    }

    /// Post request to the Node's Server
    #[tokio::main]
    pub async fn post_data<D: Serialize, T: de::DeserializeOwned>(
        &self,
        endpoint: &str,
        data: &D,
    ) -> Result<T> {
        let url_endpoint = format!("{}{}", self.url, endpoint);

        let client = Client::new();
        let response = client
            .post(url_endpoint)
            .body(serde_json::to_string(data).unwrap())
            .send()
            .await?;

        Ok(response.json::<T>().await?)
    }

    /// Patch request to the Node's Server
    #[tokio::main]
    pub async fn patch_data<D: Serialize, T: de::DeserializeOwned>(
        &self,
        endpoint: &str,
        data: &D,
    ) -> Result<T> {
        let url_endpoint = format!("{}{}", self.url, endpoint);

        let client = Client::new();
        let response = client
            .patch(url_endpoint)
            .body(serde_json::to_string(data).unwrap())
            .send()
            .await?;

        Ok(response.json::<T>().await?)
    }

    /// Get config for this node
    pub fn get_config(&self) -> Result<ConfigResponse> {
        let response = self.get_data::<ConfigResponse>("/config", None);
        let response = response.unwrap();

        Ok(response)
    }
}

#[derive(Debug, Deserialize)]
pub struct BlockRecord {
    id: String,
    created_date: String,
    modified_date: String,
    balance_key: String,
    sender: String,
    signature: String,
}

#[derive(Debug, Deserialize)]
pub struct TransactionRecord {
    id: String,
    block: BlockRecord,
    amount: u32,
    recipient: String,
    fee: Option<String>,
    memo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaginatedServerResponse<T> {
    count: u64,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<T>,
}

#[test]

fn server_get_request() {
    let node = ServerNode::new("https://bank.keysign.app", None);
    let response = node
        .get_data::<PaginatedServerResponse<TransactionRecord>>(
            "/bank_transactions",
            Some(NodeServerOptions {
                limit: 25,
                offset: 100,
            }),
        )
        .unwrap();

    println!("{:?}", response);
    assert_eq!(response.results.len(), 25);
}

#[test]
fn get_server_node_config() {
    let node = ServerNode::new("https://bank.keysign.app", None);
    let config = node.get_config().unwrap();

    assert!(config.default_transaction_fee > 0);
    assert!(!config.ip_address.is_empty());
    assert!(config.primary_validator.is_some());

    assert_eq!(config.node_type, NodeType::BANK);
    assert_eq!(config.account_number.len(), 64);
    assert_eq!(config.node_identifier.len(), 64);
}

// #[test]
// fn server_post_request() {
//     let acc = Account::new();
//     let node = ServerNode::new("https://postman-echo.com/post", None);

//     let block_data = BlockData::CoinTransfer {
//         balance_key: acc.account_number().to_string(),
//         txs: vec![],
//     };

//     let block_message = acc.create_block_message(&block_data);
//     let response = node
//         .post_data::<String, String>(
//             "/post",
//             &"This is expected to be sent back as part of response body.".to_string(),
//         )
//         .unwrap();

//     println!("{:?}", response);
//     // assert_eq!(response, acc.account_number());
// }
