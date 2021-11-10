use crate::account::{Account, NodeType};
use crate::node::common::{format_url, ClientNode, ConfigResponse};
use crate::node::server_node::ServerNode;

use reqwest::Result;
use serde::Deserialize;
// use url::Host;

/// Primary Validator
pub struct PrimaryValidator {
    /// base server
    pub server: ServerNode,
}

#[allow(dead_code)]
impl ClientNode for PrimaryValidator {
    fn new(url: &str) -> Self {
        PrimaryValidator {
            server: ServerNode::new(url, None),
        }
    }

    fn get_config(&self) -> Result<ConfigResponse> {
        self.server.get_config()
    }
}

#[derive(Debug, Deserialize)]
pub struct BalanceLockResponse {
    balance_lock: Option<String>,
}

impl PrimaryValidator {
    /// Retrieve balance lock of account from validator
    pub fn get_balance_lock<'a>(&'a self, account_number: &'a str) -> Result<BalanceLockResponse> {
        let endpoint = format!("/accounts/{}/balance_lock", account_number);
        self.server.get_data::<BalanceLockResponse>(&endpoint, None)
    }
}

#[test]
fn test_get_balance_lock() {
    let pv = PrimaryValidator::new("http://20.98.87.223/");

    let acc = Account::new();
    let BalanceLockResponse { balance_lock } = pv.get_balance_lock(acc.account_number()).unwrap();

    println!("balance_lock: {:?}", balance_lock);
}
