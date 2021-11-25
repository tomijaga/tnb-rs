use crate::{
    account::Account,
    models::{PaginatedSearchParams, SearchParams},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Enum that specifies a Node's type
#[derive(Debug, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub enum NodeType {
    /// RegularNode Node
    BANK,
    /// Primary Validator Node
    #[allow(non_camel_case_types)]
    PRIMARY_VALIDATOR,

    /// Only used in search params to specify txs that are not node fees
    NONE,
}

impl NodeType {
    /// Converts node type to string
    pub fn to_string(&self) -> String {
        match self {
            NodeType::BANK => String::from("BANK"),
            NodeType::PRIMARY_VALIDATOR => String::from("PRIMARY_VALIDATOR"),
            NodeType::NONE => String::from("NONE"),
        }
    }
}

/// Transaction Data
#[derive(Debug, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Transaction<'tx> {
    /// amount of coins to send
    pub amount: u64,

    /// The node/primary_validator processing the transaction's block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<NodeType>,

    /// optional message to add to the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'tx str>,

    /// The recipients account number
    pub recipient: &'tx str,
}

impl Transaction<'_> {
    /// Create a new transaction with only a recipient and an amount
    pub fn new<'tx>(recipient: &'tx str, amount: u64) -> Transaction<'tx> {
        Transaction {
            amount: amount,
            recipient: recipient,
            fee: None,
            memo: None,
        }
    }

    /// Create a new transaction with an additional parameter for a memo
    pub fn new_with_memo<'tx>(
        recipient: &'tx str,
        amount: u64,
        memo: &'tx str,
    ) -> Transaction<'tx> {
        Transaction {
            amount: amount,
            recipient: recipient,
            fee: None,
            memo: Some(memo),
        }
    }
}

/// Query Builder for a get transactions network request
///
/// # Field
/// - **account_number**:
///
/// # Example
///
/// ```no_run
///
///     use tnb_rs::{models::{NodeType, TransactionSearchParamsBuilder, PaginatedSearchParams}, RegularNode};
///
///     let node = RegularNode::new("https://bank.keysign.app");
///
///     let mut tx_query = TransactionSearchParamsBuilder::new();
///
///     let acc_to_search_for ="1329d3a5d4a5ec2382dc539e03f30c3760e01932834a23522d3de0393b63f224"
///     tx_query
///         .fee(Some(NodeType::BANK))
///         .limit(20)
///         .offset(0)
///         .account_number(acc_to_search_for);
///     
///     // This tx query says the node should look for the first 20 tx
///     // fees sent to a bank node that was sent or recived by the
///     // given account number
///     
///     let response = node.get_transactions(Some(&tx_query)).unwrap();
///     println!("The total number of tcs after being filtered by the query: {}", response.count);
///
///     let txs = response.results;
///     assert_eq!(txs.len(), 20);
///
/// ```
///
#[derive(Debug)]
pub struct TransactionSearchParamsBuilder<'a> {
    params: SearchParams<'a>,
}

impl<'a> PaginatedSearchParams<'a> for TransactionSearchParamsBuilder<'a> {
    /// Mutable reference to the hashmap where the data is stored
    fn get_mut_params(&mut self) -> &mut SearchParams<'a> {
        &mut self.params
    }
}

impl<'a> TransactionSearchParamsBuilder<'a> {
    /// Initialize a new transaction query builder
    pub fn new() -> Self {
        TransactionSearchParamsBuilder {
            params: SearchParams::new(),
        }
    }

    /// Returns a clone of the hasmap where all the search params are stored
    pub fn get_params(&self) -> SearchParams<'a> {
        self.params.clone()
    }

    /// Searches for transactions that were sent or received by the given account
    pub fn account_number(&mut self, account_number: &'a str) -> &mut Self {
        self.params
            .insert("account_number", account_number.to_string());
        self
    }

    /// Searches for transactions recieved by the given account
    pub fn recipient(&mut self, account_number: &'a str) -> &mut Self {
        self.params.insert("recipient", account_number.to_string());
        self
    }

    /// Searches for transactions sent by the given account
    pub fn sender(&mut self, account_number: &'a str) -> &mut Self {
        self.params.insert("sender", account_number.to_string());
        self
    }

    /// Searches for a transaction that matches the given balance_key
    pub fn balance_key(&mut self, account_number: &'a str) -> &mut Self {
        self.params
            .insert("balance_key", account_number.to_string());
        self
    }

    /// Searches for bank fees, primary validator fees or transactions with no fees
    pub fn fee(&mut self, node_type: Option<NodeType>) -> &mut Self {
        self.params.insert(
            "fee",
            match node_type {
                Some(node_with_fee) => node_with_fee.to_string(),
                _ => String::new(),
            },
        );
        self
    }

    /// Searches for a transaction that matches the given id
    pub fn id(&mut self, identifier: &'a str) -> &mut Self {
        self.params.insert("id", identifier.to_string());
        self
    }

    /// Orders the transactions according to one of the given fields
    pub fn ordering(&mut self, field: &'a str) -> &mut Self {
        self.params.insert("ordering", field.to_string());
        self
    }

    /// Removes all the data in the query builder
    pub fn clear(&mut self) {
        self.params.clear()
    }
}

#[test]

fn transaction_search_params() {
    let mut tx_query = TransactionSearchParamsBuilder::new();

    let acc1 = Account::new();
    let acc_num = acc1.account_number();
    let balance_key = acc1.signing_key();

    tx_query
        .limit(25)
        .balance_key(balance_key)
        .offset(100)
        .account_number(acc_num)
        .recipient(acc_num)
        .sender(acc_num)
        .ordering("+balance_key")
        .fee(Some(NodeType::NONE));

    // Hashmap of params is cloned on each call to the `get_params` method
    let params = tx_query.get_params();
    tx_query.recipient(acc_num);

    assert_eq!(params.get("balance_key"), Some(&balance_key.to_string()));
    assert_eq!(params.get("account_number"), Some(&acc_num.to_string()));
    assert_eq!(params.get("recipient"), Some(&acc_num.to_string()));
    assert_eq!(params.get("sender"), Some(&acc_num.to_string()));

    assert_eq!(params.get("limit"), Some(&"25".to_string()));
    assert_eq!(params.get("offset"), Some(&"100".to_string()));
    assert_eq!(params.get("fee"), Some(&"NONE".to_string()));

    tx_query.clear();
    assert!(tx_query.params.is_empty());

    // You should still be able to add params after the `clear` method is called
    tx_query.account_number(acc_num);
    assert_eq!(params.get("balance_key"), Some(&balance_key.to_string()));
}
