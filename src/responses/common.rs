use crate::models::NodeType;
use serde::{de, Deserialize};

use anyhow::{bail, Result};

#[derive(Debug, Deserialize)]
/// Block Response Data from a network node
pub struct BlockResponse {
    /// Unique id recognised only by node that stored the data
    pub id: String,

    /// The date and time when the block was created
    pub created_date: String,

    /// The date and time when the block was finalized
    pub modified_date: String,

    /// The unique id recognized by the network to prevent an account from double spending after a block has been created
    pub balance_key: String,

    /// The sender's account number
    pub sender: String,

    /// The signed block data
    pub signature: String,
}

/// Transaction Response from a network node usually returned as a paginated response
#[derive(Debug, Deserialize)]
pub struct TransactionResponse {
    /// Unique id recognised only by node that stored the data
    pub id: String,

    /// The block that this transaction is stored in
    pub block: BlockResponse,

    /// The amount of tnbc transfered in the transaction
    pub amount: u32,

    /// The recipient's account number
    pub recipient: String,

    /// Indicates whether the transaction is a fee of a node or primary validator
    pub fee: Option<String>,

    /// optional message that was added to the transaction
    pub memo: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
/// Generic Wrapper for paginated Data requested from a node
pub struct PaginatedResponse<T> {
    /// The total number of records for the search query
    pub count: u64,

    /// The link to next set of paginated data
    pub next: Option<String>,

    /// A link to the previous set of paginated data
    pub previous: Option<String>,

    /// An Array with the requested data
    pub results: Vec<T>,
}

impl<T> PaginatedResponse<T>
where
    T: de::DeserializeOwned,
{
    /// Retrieve the next set of paginated data
    /// > Returns an if you try and go out of bounds
    pub fn next(&self) -> Result<PaginatedResponse<T>> {
        self.get_paginated_data(&self.next, "Next Link is empty")
    }

    /// Retrieve the previous set of paginated data
    /// > Returns an if you try and go out of bounds
    pub fn prev(&self) -> Result<PaginatedResponse<T>> {
        self.get_paginated_data(&self.previous, "Prev link is empty")
    }

    #[tokio::main]
    async fn get_paginated_data(
        &self,
        url_option: &Option<String>,
        empty_url_error_message: &'static str,
    ) -> Result<PaginatedResponse<T>> {
        match url_option {
            Some(url) => {
                let response = reqwest::get(url).await?;
                Ok(response.json::<PaginatedResponse<T>>().await?)
            }
            None => bail!(empty_url_error_message),
        }
    }
}

/// The configuration details of a node's primary validator selection
#[derive(Debug, Deserialize)]
pub struct PrimaryValidatorConfigForNode {
    /// Account Number of the Primary Validator
    pub account_number: String,

    /// Ip Address to connect to the Primary Validator
    pub ip_address: String,

    /// The unique Identifier for the primary validator
    pub node_identifier: String,

    /// Url Port
    pub port: u16,

    /// The communication protocol of the primary validator
    pub protocol: String,

    /// Current version of the primary validator
    pub version: String,

    /// Transaction fee for processing transactions with this primary validator
    pub default_transaction_fee: u64,

    /// Link to the initialisation data of the primary validator
    pub root_account_file: String,

    /// Hash
    pub root_account_file_hash: String,

    ///
    pub seed_block_identifier: String,

    /// Fee for the primary validator's confirmation services
    pub daily_confirmation_rate: u64,

    /// Level of trust a node gives to the primary validator
    pub trust: String,
}

/// The current configuration details of a node
#[derive(Debug, Deserialize)]
pub struct ConfigResponse {
    /// - The configuration details of this node's primary validator selection
    ///
    /// - If the current node is the Primary Validator then the value of this field will be `None`
    ///
    pub primary_validator: Option<PrimaryValidatorConfigForNode>,

    /// Account Number of the Node
    pub account_number: String,

    /// Ip Address of the the Node
    pub ip_address: String,

    /// The unique Identifier for the node
    pub node_identifier: String,

    /// Url Port
    pub port: u16,

    /// The communication protocol of the node
    pub protocol: String,

    /// Current Node version
    pub version: String,

    /// Transaction fee for processing transactions through this node
    pub default_transaction_fee: u64,

    /// The node type
    pub node_type: NodeType,
}
