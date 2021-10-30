use crate::account::*;

use bip39::{Language, Mnemonic};
use ed25519_dalek_bip32::{DerivationPath, ExtendedSecretKey};
use hex;
use rand;

/// A Hierarchical Deterministic Wallet that can create an infinite number of Accounts from a single mnemonic phrase
///
/// To specify the location of accounts in the tree we use something called path a path
///
/// This wallet uses the bip44 standard for compatibility with other existing wallets
/// Therefor all paths are formatted like this path format:  `m / 44' / coin_type' / account_index' / 0' / address_index'`
///
/// The coin_type is specific to the cryptocurrency network and thenewboston's is `2002`
///
/// so the paths are `m / 44' / 2002' / account_index' / 0' / address_index'`
///
/// The account_index and address_index can be changed by the user as they please to get their desired account
///
/// With this format a user can still get up to 4,611,686,014,132,420,600 accounts from a single mnemonic phrase

#[derive(Debug)]
pub struct HDWallet {
    pub mnemonic: String,
    pub seed: Vec<u8>,
    pub master_key: ExtendedSecretKey,
}

#[allow(dead_code)]
impl HDWallet {
    /// Creates a new HDWallet
    ///
    /// ```
    ///     let hd = HDWallet::new();
    /// ```
    ///  You should print out you mnemonic phrase and save it somewhere secure so you can use it again later
    ///
    /// ```
    ///     println!("mnemonic: {}", hd.mnemonic);
    /// ```
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap();
        HDWallet::from_mnemonic(mnemonic.to_string(), None)
    }

    /// ```
    /// let hd = HDWallet::from_mnemonic(saved_mnemonic);
    /// ```
    pub fn from_mnemonic(mnemonic: String, password: Option<String>) -> Self {
        let m = Mnemonic::parse_normalized(&mnemonic)
            .ok()
            .expect("Invalid Mnemonic");

        let seed = m
            .to_seed(match password {
                Some(pw) => pw,
                None => String::new(),
            })
            .to_vec();
        let xpriv = ExtendedSecretKey::from_seed(&seed).unwrap();

        HDWallet {
            mnemonic: m.to_string(),
            seed: seed,
            master_key: xpriv,
        }
    }

    pub fn from_seed(seed: String) -> Self {
        let seed_as_bytes = hex::decode(&seed).unwrap();
        let xpriv = ExtendedSecretKey::from_seed(&seed_as_bytes).unwrap();

        HDWallet {
            mnemonic: "".to_string(),
            seed: seed_as_bytes,
            master_key: xpriv,
        }
    }

    fn get_account_from_path(&self, path: &str) -> Account {
        let child_xpriv: ExtendedSecretKey = self
            .master_key
            .derive::<DerivationPath>(&path.parse().unwrap())
            .ok()
            .expect("Invalid Derivation Path");

        Account::from_private_key(&hex::encode(child_xpriv.secret_key))
    }

    /// Retrieves the account in the determinstic tree specified by the account_index and address_index
    ///
    /// `m / purpose' / coin_type' / account_index' / change' / address_index'`
    ///
    /// # Example
    /// ```
    ///     let hd = HDWallet::from_mnemonic("visa nephew like this amazing soldier negative front elevator warfare teach good");
    ///
    ///     let acc0 = hd.get_account(0, 0);
    ///     let acc1 = hd.get_account(0, 1);
    ///
    ///
    /// ```
    pub fn get_account(&self, account_index: u32, address_index: u32) -> Account {
        let path = format!("m/44'/2002'/{}'/0'/{}'", account_index, address_index);

        self.get_account_from_path(&path)
    }

    /// Retrieves account from the given address_index on the column where (account_index = 0)
    pub fn get_account_from_first_column(&self, address_index: u32) -> Account {
        self.get_account(0, address_index)
    }

    /// Retrieves the first account from the hierarchical deterministic tree of accounts
    /// ```
    /// let first_account = hd.first_account();
    ///
    /// assert_eq!(first_account.account_number, self.get_account(0, 0).account_number);
    /// assert_eq!(first_account.private_key, self.get_account(0, 0).private_key);
    ///
    ///
    /// ```
    ///
    pub fn first_account(&self) -> Account {
        self.get_account(0, 0)
    }

    /// Returns the seed in hex format
    pub fn seed_hex(&self) -> String {
        hex::encode(&self.seed)
    }
}
