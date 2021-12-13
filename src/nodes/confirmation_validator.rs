use crate::account::Account;
use crate::{
    nodes::{
        server_node::ServerNode,
        traits::{ServerNodeTrait, ValidatorTrait},
    },
    responses::{AccountBalanceLockResponse, AccountBalanceResponse, ConfigResponse},
};

use reqwest::Result;

/// Confirmation Validator
#[derive(Debug)]
pub struct ConfirmationValidator {
    /// base server
    base: ServerNode,
}

#[allow(dead_code)]
impl ServerNodeTrait for ConfirmationValidator {
    fn get_base(&self) -> &ServerNode {
        &self.base
    }
}

impl ValidatorTrait for ConfirmationValidator {}

impl ConfirmationValidator {
    /// Create a new confirmation validator instance
    ///
    /// ```
    ///     use tnb_rs::{nodes::ConfirmationValidator};
    ///
    ///     let cv = ConfirmationValidator::new("http://54.241.48.170");
    ///
    /// ```
    pub fn new(url: &str) -> Self {
        ConfirmationValidator {
            base: ServerNode::new(url),
        }
    }
}
