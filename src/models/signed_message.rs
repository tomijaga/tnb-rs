use serde::{Deserialize, Serialize};

/// Enum for Supported Node Requests
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ChainData {
    /// Request structure for changing an account's trust
    UpdateAccountTrust {
        /// account's trust
        trust: i32,
    },
}

/// Structure for making Node requests to the network
#[derive(Debug, Serialize)]
pub struct SignedMessage<'a> {
    /// message that contains the node's request
    pub message: &'a ChainData,

    /// The node's identification number
    pub node_identifier: &'a str,

    /// thh signed message
    pub signature: String,
}
