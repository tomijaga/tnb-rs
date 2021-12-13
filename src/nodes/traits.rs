use crate::{
    nodes::server_node::ServerNode,
    responses::{AccountBalanceLockResponse, AccountBalanceResponse, ConfigResponse},
};
use reqwest::Result;

/// A trait for implementing nodes using the server node as a base layer.
pub trait ServerNodeTrait {
    /// Retrieves the base server so you can avoid implenting generic methods
    fn get_base(&self) -> &ServerNode;

    /// Get the config details of a node
    fn get_config(&self) -> Result<ConfigResponse>
    where
        Self: Sized,
    {
        self.get_base().get_config()
    }
}

///  A trait for implementing validator nodes using the server node as a base layer.
pub trait ValidatorTrait: ServerNodeTrait {
    /// Retrieve the number of coins in an account
    fn get_account_balance<'a>(
        &'a self,
        account_number: &'a str,
    ) -> Result<AccountBalanceResponse> {
        let endpoint = format!("/accounts/{}/balance", account_number);
        self.get_base()
            .get_data::<AccountBalanceResponse>(&endpoint, None)
    }

    /// Get the balance lock of an account's next transaction
    fn get_account_balance_lock<'a>(
        &'a self,
        account_number: &'a str,
    ) -> Result<AccountBalanceLockResponse> {
        let endpoint = format!("/accounts/{}/balance_lock", account_number);
        self.get_base()
            .get_data::<AccountBalanceLockResponse>(&endpoint, None)
    }
}
