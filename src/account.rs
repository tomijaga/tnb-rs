use anyhow::{ensure, Result};
use hex;
use sodiumoxide::crypto::sign;
use sodiumoxide::crypto::sign::ed25519::{
    sign_detached, verify_detached, PublicKey, SecretKey, Seed, Signature,
};
use std::{convert::TryInto, fmt};

use crate::models::{BlockMessage, BlockType, ChainData, SignedMessage};

fn convert_vec_to_fixed_array<T, const N: usize>(vec: Vec<T>) -> [T; N] {
    vec.try_into().unwrap_or_else(|v: Vec<T>| {
        panic!(
            "Expected a hex of length {} but it was {}",
            N / 2,
            v.len() / 2
        )
    })
}

fn hex_to_fixed_bytes<const N: usize>(hex_key: &str) -> [u8; N] {
    let key_as_bytes = hex::decode(hex_key).ok().expect("Hex Key is invalid");
    convert_vec_to_fixed_array::<u8, N>(key_as_bytes)
}

/// An Account consists of an account number and a signing key.
///
/// - The account number is used to identify an account on thenewboston network
/// - The signing key is used to prove that you are the owner of the account by creating signatures and validating them with your account number.
/// For this reason, your signing key should not be shared with anyone.
///
#[derive(PartialEq, Eq)]
pub struct Account {
    signing_key_bytes: SecretKey,
    account_number_hex: String,
    signing_key_hex: String,
}

impl fmt::Debug for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Account")
            .field("Account Number", &self.account_number())
            .field("Signing Key", &self.signing_key())
            .finish()
    }
}
#[allow(dead_code)]
impl Account {
    fn create(keypair: (PublicKey, SecretKey)) -> Self {
        let (pk, sk) = keypair;
        let signing_key_as_bytes = sk.0;

        Account {
            signing_key_bytes: sk,
            account_number_hex: hex::encode(pk),
            signing_key_hex: hex::encode(&signing_key_as_bytes[0..32]),
        }
    }

    /// Creates an random Account
    ///
    /// ```
    ///     use tnb_rs::Account;
    ///
    ///     let acc = Account::new();
    ///
    ///     // store you signing key safely before you proceed
    ///     println!("siging key: {}", acc.signing_key());
    ///
    /// ```
    pub fn new() -> Self {
        Account::create(sign::gen_keypair())
    }

    /// Creates an Account from a signing key
    /// # Example
    ///
    /// ```
    ///     use tnb_rs::Account;
    ///
    ///     let signing_key = "8cf08eb96b00b5a4df86a750bb7ae595a9dbbe91fc091463bfb3d950d5dac467";
    ///     let acc = Account::from_signing_key(signing_key).unwrap();
    ///
    ///     assert_eq!(acc.signing_key(), signing_key);
    ///
    /// ```
    pub fn from_signing_key(signing_key_hex: &str) -> Result<Self> {
        let result = hex::decode(signing_key_hex);
        ensure!(result.is_ok(), result.err().unwrap());

        let signing_key_as_bytes = result.unwrap();
        let result = Seed::from_slice(&signing_key_as_bytes);
        ensure!(
            result.is_some(),
            format!(
                "Signing Key hex needs to be of length 64 but only found {}",
                signing_key_as_bytes.len() * 2
            )
        );
        let priv_key_as_seed = result.unwrap();
        let keypair = sign::keypair_from_seed(&priv_key_as_seed);
        Ok(Account::create(keypair))
    }

    /// Returns the account number as a hex string
    pub fn account_number(&self) -> &str {
        &self.account_number_hex
    }

    /// Returns the signing key as a hex string
    pub fn signing_key(&self) -> &str {
        &self.signing_key_hex
    }

    /// Returns a tuple of the account number and signing key
    pub fn keypair_as_hex(&self) -> (&str, &str) {
        (self.account_number(), self.signing_key())
    }

    /// Checks if a signing key and account number are keypairs
    ///
    /// # Example
    /// ```rust
    ///     
    ///     use tnb_rs::Account;
    ///
    ///     let signing_key = "8cf08eb96b00b5a4df86a750bb7ae595a9dbbe91fc091463bfb3d950d5dac467";
    ///     let acc = Account::from_signing_key(signing_key).unwrap();
    ///     let (acc_num, sk ) = acc.keypair_as_hex();
    ///     
    ///     assert_eq!(signing_key, sk);
    ///
    ///     let is_keypair = Account::is_valid_keypair(&sk, &acc_num);
    ///     assert_eq!(is_keypair, true);
    ///     
    /// ```
    ///
    pub fn is_valid_keypair(_signing_key_hex: &str, _account_number_hex: &str) -> bool {
        let acc = Account::from_signing_key(_signing_key_hex).unwrap();

        acc.account_number().eq(_account_number_hex)
    }

    /// Signs the given message with the Account's signing key
    ///
    /// # Returns
    /// A `signature` of the signed message in form of a hex string.
    ///
    ///
    pub fn create_signature(&self, message: &str) -> String {
        let message_as_bytes = message.as_bytes();
        let signed_message = sign_detached(message_as_bytes, &self.signing_key_bytes);
        hex::encode(signed_message)
    }

    /// Static method that the verifies that a message was signed by a specific account number.
    ///
    /// # Inputs
    /// - `signature` being verified
    /// - the original `message`
    /// - the signer's `account number`
    ///
    /// # Returns
    /// A `signature` in form of a hex string
    ///

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

    /// Creates a block message that can be broadcasted to make changes to an account on the network
    pub fn create_block_message<'a>(&'a self, data: &'a BlockType) -> BlockMessage {
        let serialized_block = serde_json::to_string(&data);
        BlockMessage {
            account_number: self.account_number(),
            message: data,
            signature: self.create_signature(&serialized_block.unwrap()),
        }
    }

    /// Creates a message that nodes can broadcast to make changes on the network
    pub fn create_signed_message<'a>(&'a self, data: &'a ChainData) -> SignedMessage {
        let serialized_data = serde_json::to_string(&data);
        SignedMessage {
            message: data,
            node_identifier: self.account_number(),
            signature: self.create_signature(&serialized_data.unwrap()),
        }
    }
}
