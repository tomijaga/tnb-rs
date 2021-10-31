use crate::account::*;

use bip39::{Language, Mnemonic};
use ed25519_dalek_bip32::{DerivationPath, ExtendedSecretKey};
use hex;
use rand;

/// The max number for the account_index and address_index
pub const MAX_CHILD_INDEX: u32 = 2_147_483_647;

/// A Hierarchical Deterministic Wallet that can create an infinite number of Accounts from a single mnemonic phrase.
///
/// To specify the location of accounts in the tree we use something called path a path
///
/// This wallet uses the bip44 standard for compatibility with other existing wallets
/// Therefor all paths are formatted like this path format:  
///
/// ```text
/// m / 44' / coin_type' / account_index' / 0' / address_index'
/// ```
///
/// The coin_type is specific to the cryptocurrency network and thenewboston's is `2002`
///
/// so the paths are
/// ```text
///     m / 44' / 2002' / account_index' / 0' / address_index'
/// ```
///
/// The `account_index` and `address_index` can be changed by the user as they please to get their desired account
///
/// With this format a user can still get up to ```4,611,686,014,132,420,600``` accounts from a single mnemonic phrase

#[derive(Debug)]
pub struct HDWallet {
    /// mnemonic phrase
    pub mnemonic: String,
    seed: Vec<u8>,
    master_key: ExtendedSecretKey,
}

#[allow(dead_code)]
impl HDWallet {
    /// Creates a new HD Wallet
    ///
    /// ```
    ///     use tnb_rs::HDWallet;
    ///     let hd = HDWallet::new();
    ///     println!("mnemonic: {}", hd.mnemonic);
    /// ```
    ///  Ensure that you save your mnemonic phrase somewhere secure so you can use it again later
    ///
    ///

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap();
        HDWallet::from_mnemonic(mnemonic.to_string().as_ref(), None)
    }

    /// Creates a HD Wallet from a specified mnemonic phrase.
    ///
    /// # Inputs
    ///     
    /// - `mnemonic phrase` - a 12, 18, or 24 random words generated from a wordlist
    /// - `password` - an optional string to further secure your wallet in case your mnemonic phrase is ever exposed
    ///
    /// # Example
    /// ```
    ///     use tnb_rs::HDWallet;
    ///
    ///     let mnemonic = "visa nephew like this amazing soldier negative front elevator warfare teach good";
    ///     let hd1 = HDWallet::from_mnemonic(mnemonic, None);
    ///
    ///     let hd2 = HDWallet::from_mnemonic(mnemonic, Some("password"));
    ///
    ///     // same mnemonic phrase
    ///     assert_eq!(hd1.mnemonic, hd2.mnemonic);
    ///
    ///     // but different seeds meaning all the accounts generated will be different too.   
    ///     assert_ne!(hd1.seed_hex(), hd2.seed_hex());
    ///
    /// ```
    ///
    ///
    pub fn from_mnemonic(mnemonic: &str, password: Option<&str>) -> Self {
        let m = Mnemonic::parse_normalized(&mnemonic)
            .ok()
            .expect("Invalid Mnemonic");

        let seed = m
            .to_seed(match password {
                Some(pw) => pw,
                None => "",
            })
            .to_vec();
        let xpriv = ExtendedSecretKey::from_seed(&seed).unwrap();

        HDWallet {
            mnemonic: m.to_string(),
            seed: seed,
            master_key: xpriv,
        }
    }

    /// Creates a new HD Wallet with an optional field for a password
    pub fn new_with_password(password: Option<&str>) -> Self {
        let mut rng = rand::thread_rng();
        let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap();
        HDWallet::from_mnemonic(mnemonic.to_string().as_ref(), password)
    }

    /// Creates a HD Wallet from a seed in hex string format
    ///
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

        Account::from_signing_key(&hex::encode(child_xpriv.secret_key))
    }

    /// Retrieves the account specified by the account_index and address_index from the HD Wallet
    ///
    /// # Example
    /// ```
    ///     use tnb_rs::{HDWallet, MAX_CHILD_INDEX};
    ///     
    ///     let mnemonic = "visa nephew like this amazing soldier negative front elevator warfare teach good";
    ///     let hd = HDWallet::from_mnemonic(mnemonic, None);
    ///     
    ///     let acc1 = hd.get_account(0, 0);
    ///     let acc2 = hd.get_account(23, 1_000);
    ///
    ///     let last_account = hd.get_account(MAX_CHILD_INDEX, MAX_CHILD_INDEX);
    ///
    /// ```
    ///
    ///
    pub fn get_account(&self, account_index: u32, address_index: u32) -> Account {
        let path = format!("m/44'/2002'/{}'/0'/{}'", account_index, address_index);
        self.get_account_from_path(&path)
    }

    /// Retrieves an account specified by the address_index from the column where (account_index == 0)
    pub fn get_account_0(&self, address_index: u32) -> Account {
        self.get_account(0, address_index)
    }

    /// Retrieves the first account from the hd wallet
    ///
    /// # Example
    /// ```
    ///     use tnb_rs::HDWallet;
    ///     
    ///     let mnemonic = "visa nephew like this amazing soldier negative front elevator warfare teach good";
    ///     let hd = HDWallet::from_mnemonic(mnemonic, None);
    ///
    ///     let get_first_account = hd.get_first_account();
    ///
    ///     assert_eq!(get_first_account.account_number_hex(),
    ///                 hd.get_account(0, 0).account_number_hex());
    ///     assert_eq!(get_first_account.signing_key_hex(),
    ///                 hd.get_account(0, 0).signing_key_hex());
    ///
    ///
    /// ```
    ///
    pub fn get_first_account(&self) -> Account {
        self.get_account(0, 0)
    }

    /// Returns the seed in hex format
    pub fn seed_hex(&self) -> String {
        hex::encode(&self.seed)
    }
}

#[test]
fn test_vectors_for_ed25519() {
    // Slip 10 Test vectors for ed25519
    // https://github.com/satoshilabs/slips/blob/master/slip-0010.md#test-vector-2-for-ed25519

    let seed =
        String::from(
            "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542"
        );

    let hd = HDWallet::from_seed(seed);

    let acc = hd.get_account_from_path("m/0'");
    assert_eq!(
        acc.account_number_hex(),
        "86fab68dcb57aa196c77c5f264f215a112c22a912c10d123b0d03c3c28ef1037"
    );
    assert_eq!(
        acc.signing_key_hex(),
        "1559eb2bbec5790b0c65d8693e4d0875b1747f4970ae8b650486ed7470845635"
    );
    let acc = hd.get_account_from_path("m/0'/2147483647'");
    assert_eq!(
        acc.account_number_hex(),
        "5ba3b9ac6e90e83effcd25ac4e58a1365a9e35a3d3ae5eb07b9e4d90bcf7506d"
    );
    assert_eq!(
        acc.signing_key_hex(),
        "ea4f5bfe8694d8bb74b7b59404632fd5968b774ed545e810de9c32a4fb4192f4"
    );
    let acc = hd.get_account_from_path("m/0'/2147483647'/1'/2147483646'/2'");
    assert_eq!(
        acc.account_number_hex(),
        "47150c75db263559a70d5778bf36abbab30fb061ad69f69ece61a72b0cfa4fc0"
    );
    assert_eq!(
        acc.signing_key_hex(),
        "551d333177df541ad876a60ea71f00447931c0a9da16f227c11ea080d7391b8d"
    );
}
