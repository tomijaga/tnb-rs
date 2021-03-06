use reqwest::{header::CONTENT_TYPE, Client, Result};
use serde::{de, Serialize};
use url::{Origin, Url};

use crate::{
    models::{NodeType, SearchParams},
    responses::{ConfigResponse, PaginatedResponse, TransactionResponse},
};

/// Base Api for a node
#[allow(dead_code)]
#[derive(Debug)]
pub struct ServerNode {
    /// url address of node's server
    pub url: String,
}

#[allow(dead_code)]
impl ServerNode {
    /// Get request to the Node's Server
    #[tokio::main]
    pub async fn get_data<T: de::DeserializeOwned>(
        &self,
        endpoint: &str,
        query: Option<SearchParams>,
    ) -> Result<T> {
        let url_endpoint = format!("{}{}", self.url, endpoint);

        let url = match query {
            Some(query_params) => {
                Url::parse_with_params(&url_endpoint, query_params.iter()).unwrap()
            }
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
            .header(CONTENT_TYPE, "application/json")
            .json(&data)
            .body(serde_json::to_string(data).unwrap())
            .send()
            .await?;

        let r = &response;
        println!("\npost response: {:?}, text: ", r.status(),);

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
}

impl ServerNode {
    /// Initialize a new server node
    pub fn new(url: &str) -> Self {
        let parsed_url = Url::parse(url).unwrap();

        let origin = parsed_url.origin();

        let formatted_url = match origin {
            Origin::Tuple(protocol, host, port) => format!("{}://{}:{}", protocol, host, port),
            _ => String::new(),
        };

        ServerNode { url: formatted_url }
    }

    /// Get config for this node
    pub fn get_config(&self) -> Result<ConfigResponse> {
        let response = self.get_data::<ConfigResponse>("/config", None);
        let response = response.unwrap();

        Ok(response)
    }
}

#[test]
fn server_get_request() {
    let node = ServerNode::new("https://bank.keysign.app");
    let response = node
        .get_data::<PaginatedResponse<TransactionResponse>>(
            "/bank_transactions",
            Some(SearchParams::from([
                ("limit", "25".to_string()),
                ("offset", "100".to_string()),
            ])),
        )
        .unwrap();

    println!("{:?}", response);
    assert_eq!(response.results.len(), 25);
}

#[test]
fn get_server_node_config() {
    let node = ServerNode::new("https://bank.keysign.app");
    let config = node.get_config().unwrap();

    assert!(config.default_transaction_fee > 0);
    assert!(!config.ip_address.is_empty());
    assert!(config.primary_validator.is_some());

    assert_eq!(config.node_type, NodeType::BANK);
    assert_eq!(config.account_number.len(), 64);
    assert_eq!(config.node_identifier.len(), 64);
}
