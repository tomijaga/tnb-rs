use crate::account::NodeType;
use crate::node::server_node::ServerNode;

use anyhow::ensure;
use reqwest::Result;
use serde::Deserialize;
// use url::Host;

/// Config for a selected primary validator
#[derive(Debug, Deserialize)]
pub struct PrimaryValidatorConfig {
    /// Account Number of the Primary Validator
    pub account_number: String,

    /// Ip Address to connect to the Primary Validator
    pub ip_address: String,

    /// The unique Identifier for the primary validator
    pub node_identifier: String,

    /// Url Port
    pub port: u16,

    /// protocol
    pub protocol: String,

    /// Current version of the Primary Validator
    pub version: String,

    /// Transaction fee for processing transactions with this primary validator
    pub default_transaction_fee: u64,

    /// Link to the initialisation data of the primary validator
    pub root_account_file: String,

    /// Hash
    pub root_account_file_hash: String,

    /// ??
    pub seed_block_identifier: String,

    /// Fee for the primary validator's confirmation services
    pub daily_confirmation_rate: u64,

    /// Level of trust this node gives to the primary validator
    pub trust: String,
}

/// Config Response for a Node
#[derive(Debug, Deserialize)]
pub struct ConfigResponse {
    /// The Config of the the Primary Validator selected by the Node
    ///
    /// > The value of this field is `None` if the current node is the Primary Validator
    pub primary_validator: Option<PrimaryValidatorConfig>,

    /// Account Number of the Node
    pub account_number: String,

    /// Ip Address to connect to the Node
    pub ip_address: String,

    /// The unique Identifier for the node
    pub node_identifier: String,

    /// Url Port
    pub port: u16,

    /// protocol
    pub protocol: String,

    /// Current Node version
    pub version: String,

    /// Transaction fee for processing transactions with this node
    pub default_transaction_fee: u64,

    /// The type of this node
    pub node_type: NodeType,
}

pub trait ClientNode {
    fn new(url: &str) -> Self;
    fn get_config(&self) -> Result<ConfigResponse>
    where
        Self: Sized;
}

#[allow(dead_code)]
pub fn format_url(protocol: &str, host: &str, port: u16) -> String {
    format!("{}://{}:{}", protocol, host, port)
}
