use crate::account::*;

use bip39::{Language, Mnemonic};
use ed25519_dalek_bip32::{DerivationPath, ExtendedSecretKey};
use hex;
use rand;

#[derive(Debug)]
pub struct HDWallet {
    pub mnemonic: String,
    seed: [u8; 64],
}

#[allow(dead_code)]
impl HDWallet {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap();
        HDWallet {
            mnemonic: mnemonic.to_string(),
            seed: mnemonic.to_seed(""),
        }
    }

    pub fn from_mnemonic(mnemonic: String, password: Option<String>) -> Self {
        let m = Mnemonic::parse_normalized(&mnemonic)
            .ok()
            .expect("Invalid Mnemonic");

        HDWallet {
            mnemonic: m.to_string(),
            seed: m.to_seed(match password {
                Some(pw) => pw,
                None => String::new(),
            }),
        }
    }

    pub fn from_seed(seed: String) -> Self {
        let seed_as_bytes = hex_to_fixed_bytes::<64>(&seed);

        HDWallet {
            mnemonic: "".to_string(),
            seed: seed_as_bytes,
        }
    }

    fn get_account_from_path(&self, path: &str) -> Account {
        let xpriv = ExtendedSecretKey::from_seed(&self.seed).unwrap();

        let child_xpriv: ExtendedSecretKey = xpriv
            .derive::<DerivationPath>(&path.parse().unwrap())
            .ok()
            .expect("Invalid Derivation Path");

        Account::from_private_key(&hex::encode(child_xpriv.secret_key))
    }

    pub fn get_account(&self, account_index: u32, address_index: u32) -> Account {
        let path = format!("m/44'/2002'/{}'/0'/{}'", account_index, address_index);

        self.get_account_from_path(&path)
    }

    pub fn get_account_from_first_column(&self, address_index: u32) -> Account {
        self.get_account(0, address_index)
    }

    pub fn first_account(&self) -> Account {
        self.get_account(0, 0)
    }

    pub fn seed_hex(&self) -> String {
        hex::encode(self.seed)
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
        acc.private_key_hex(),
        "1559eb2bbec5790b0c65d8693e4d0875b1747f4970ae8b650486ed7470845635"
    );
    let acc = hd.get_account_from_path("m/0'/2147483647'");
    assert_eq!(
        acc.account_number_hex(),
        "5ba3b9ac6e90e83effcd25ac4e58a1365a9e35a3d3ae5eb07b9e4d90bcf7506d"
    );
    assert_eq!(
        acc.private_key_hex(),
        "ea4f5bfe8694d8bb74b7b59404632fd5968b774ed545e810de9c32a4fb4192f4"
    );
    let acc = hd.get_account_from_path("m/0'/2147483647'/1'/2147483646'/2'");
    assert_eq!(
        acc.account_number_hex(),
        "47150c75db263559a70d5778bf36abbab30fb061ad69f69ece61a72b0cfa4fc0"
    );
    assert_eq!(
        acc.private_key_hex(),
        "551d333177df541ad876a60ea71f00447931c0a9da16f227c11ea080d7391b8d"
    );
}
