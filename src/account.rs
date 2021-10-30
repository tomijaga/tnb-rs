use hex;
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::sign;
use sodiumoxide::crypto::sign::ed25519::{
    sign_detached, verify_detached, PublicKey, SecretKey, Seed, Signature,
};
use std::convert::TryInto;

pub fn convert_vec_to_fixed_array<T, const N: usize>(vec: Vec<T>) -> [T; N] {
    vec.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn hex_to_fixed_bytes<const N: usize>(hex_key: &str) -> [u8; N] {
    let key_as_bytes = hex::decode(hex_key).ok().expect("Hex Key is invalid");
    convert_vec_to_fixed_array::<u8, N>(key_as_bytes)
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Node {
    BANK,
    #[allow(non_camel_case_types)]
    PRIMARY_VALIDATOR,
    None,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub amount: u64,
    pub recipient: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<Node>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlockData {
    CoinTransfer {
        balance_key: String,
        txs: Vec<Transaction>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockMessage {
    pub account_number: String,
    pub message: BlockData,
    pub signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ChainData {
    UpdateAccountTrust { trust: i32 },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignedMessage {
    message: ChainData,
    node_identifier: String,
    signature: String,
}

/// An Account consists of an account_number and private_key.
///
/// This private_key is used for signing transactions on chain to prove that you are the owner of the account
/// and should not be shared with anyone.
#[derive(Debug)]
pub struct Account {
    pub account_number: PublicKey,
    pub private_key: SecretKey,
}

#[allow(dead_code)]
impl Account {
    /// Creates an random Account
    ///
    /// ```
    /// use tnb_rs::Account;
    /// let acc = Account::new();
    ///
    /// ```
    pub fn new() -> Self {
        let (pk, sk) = sign::gen_keypair();
        Account {
            account_number: pk,
            private_key: sk,
        }
    }

    /// Creates an Account from a private_key
    /// # Example
    ///
    /// ```
    ///     use tnb_rs::Account;
    ///
    ///     fn main(){
    ///
    ///     let private_key = "8cf08eb96b00b5a4df86a750bb7ae595a9dbbe91fc091463bfb3d950d5dac467";
    ///     let acc = Account::from_private_key(private_key);
    ///
    ///     assert_eq!(acc.private_key, private_key);
    ///
    ///     }
    /// ```
    pub fn from_private_key(private_key_hex: &str) -> Self {
        let private_key_as_bytes: [u8; 32] = hex_to_fixed_bytes::<32>(private_key_hex);
        let priv_key_as_seed = Seed(private_key_as_bytes);
        let (pk, sk) = sign::keypair_from_seed(&priv_key_as_seed);

        Account {
            account_number: pk,
            private_key: sk,
        }
    }

    /// Returns the account number as a hex string
    pub fn account_number_hex(&self) -> String {
        hex::encode(self.account_number)
    }

    /// Returns the private key as a hex string
    pub fn private_key_hex(&self) -> String {
        hex::encode(&self.private_key[0..32])
    }

    /// Checks if a private key and account number are a keypair
    pub fn is_valid_keypair(_private_key_hex: &str, _account_number_hex: &str) -> bool {
        let acc = Account::from_private_key(_private_key_hex);
        _account_number_hex == acc.account_number_hex()
    }

    /// Signs the given message with the Account's private key
    pub fn create_signature(&self, message: &str) -> String {
        let message_as_bytes = message.as_bytes();
        let signed_message = sign_detached(message_as_bytes, &self.private_key);
        hex::encode(signed_message)
    }

    /// Verifies that the signature and message are a
    pub fn verify_signature(
        signature_as_hex: &str,
        message: &str,
        account_number_hex: &str,
    ) -> bool {
        let signature = hex_to_fixed_bytes::<64>(signature_as_hex);
        let account_number = hex_to_fixed_bytes::<32>(account_number_hex);

        verify_detached(
            &Signature::new(signature),
            message.as_bytes(),
            &PublicKey(account_number),
        )
    }

    /// Creates a block message to make a transaction on thenewboston network
    pub fn create_block_message(&self, data: BlockData) -> BlockMessage {
        let serialized_block = serde_json::to_string(&data);
        BlockMessage {
            account_number: self.account_number_hex(),
            message: data,
            signature: self.create_signature(serialized_block.unwrap().as_ref()),
        }
    }

    /// Creates a message that nodes can use to make changes on the network
    pub fn create_signed_message(&self, data: ChainData) -> SignedMessage {
        let serialized_data = serde_json::to_string(&data);
        SignedMessage {
            message: data,
            node_identifier: self.account_number_hex(),
            signature: self.create_signature(serialized_data.unwrap().as_ref()),
        }
    }
}
