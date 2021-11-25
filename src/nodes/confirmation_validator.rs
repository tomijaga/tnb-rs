use crate::account::Account;
use crate::{
    nodes::{
        server_node::ServerNode,
        traits::{ServerNodeTrait, ValidatorTrait},
    },
    responses::{AccountBalanceLockResponse, AccountBalanceResponse, ConfigResponse},
};

use reqwest::Result;
// use url::Host;

/// Confirmation Validator
#[derive(Debug)]
pub struct ConfirmationValidator {
    /// base server
    pub base: ServerNode,
}

#[allow(dead_code)]
impl ServerNodeTrait for ConfirmationValidator {
    fn get_base(&self) -> &ServerNode {
        &self.base
    }
}

impl ValidatorTrait for ConfirmationValidator {}

impl ConfirmationValidator {
    pub fn new(url: &str) -> Self {
        ConfirmationValidator {
            base: ServerNode::new(url),
        }
    }
}
