use crate::{
    nodes::{
        server_node::ServerNode,
        traits::{ServerNodeTrait, ValidatorTrait},
    },
    responses::{AccountBalanceLockResponse, AccountBalanceResponse, ConfigResponse},
};

use reqwest::Result;
// use url::Host;

/// Primary Validator
#[derive(Debug)]
pub struct PrimaryValidator {
    /// base server
    pub base: ServerNode,
}

#[allow(dead_code)]
impl ServerNodeTrait for PrimaryValidator {
    fn get_base(&self) -> &ServerNode {
        &self.base
    }
}

impl ValidatorTrait for PrimaryValidator {}

impl PrimaryValidator {
    pub fn new(url: &str) -> Self {
        PrimaryValidator {
            base: ServerNode::new(url),
        }
    }
}
