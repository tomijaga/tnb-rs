use serde::Deserialize;

/// Validator's response to the `/balance_lock` endpoint
#[derive(Debug, Deserialize)]
pub struct AccountBalanceLockResponse {
    /// The unique id for the account's next transaction
    pub balance_lock: Option<String>,
}

/// Validator's response to the `/balance` endpoint
#[derive(Debug, Deserialize)]
pub struct AccountBalanceResponse {
    /// The number of coins stored in the account
    pub balance: Option<u64>,
}
