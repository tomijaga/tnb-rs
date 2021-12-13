use tnb_rs::models::*;
use tnb_rs::*;

const SIGNING_KEY_HEX: &str = "4b3e69add153435a30c03f6ba4576cedeacfd9d362272a39863f0f3e37eda72c";
const ACCOUNT_NUMBER_HEX: &str = "1329d3a5d4a5ec2382dc539e03f30c3760e01932834a23522d3de0393b63f224";

#[test]
fn generates_random_account() {
    let acc = Account::new();
    assert_eq!(acc.account_number().len(), 64);
    assert_eq!(acc.signing_key().len(), 64);
}
#[test]
fn generate_account_from_invalid_signing_key() {
    let acc = Account::from_signing_key("abcdefghijklmnop");
    assert_eq!(acc.is_err(), true);
}

#[test]
fn generate_account_from_valid_signing_key() {
    let acc = Account::from_signing_key(SIGNING_KEY_HEX);
    assert_eq!(acc.is_ok(), true);

    let acc = acc.unwrap();
    assert_eq!(acc.account_number().len(), 64);
    assert_eq!(acc.signing_key().len(), 64);

    assert_eq!(acc.account_number(), ACCOUNT_NUMBER_HEX);
    assert_eq!(acc.signing_key(), SIGNING_KEY_HEX);
}

#[test]
fn is_valid_keypair() {
    assert_eq!(
        Account::is_valid_keypair(SIGNING_KEY_HEX, ACCOUNT_NUMBER_HEX),
        true
    );
    assert_eq!(
        Account::is_valid_keypair(ACCOUNT_NUMBER_HEX, SIGNING_KEY_HEX),
        false
    );
}

#[test]

fn create_signature() {
    let acc = Account::from_signing_key(SIGNING_KEY_HEX).unwrap();

    let sig = acc.create_signature("signature");

    let correct_signature = "c41031b2a948e8deb82ce97028fd05b64c9da1e64081c45fdc42ee1a4f837858d0a6cdfc1a52bee4b02cca57d3ba3e28590564f8efda36287441332824088a0f";
    println!(" sig: {:?}", sig);
    assert_eq!(sig, correct_signature);
}

#[test]
fn create_tx_signature() {
    let acc = Account::from_signing_key(SIGNING_KEY_HEX).unwrap();

    let tx = Transaction {
        amount: 1,
        recipient: "000000000000000000000000000000000000000000000000000000000000dead",
        memo: None,
        fee: None,
    };
    let sig = acc.create_signature(&serde_json::to_string(&tx).unwrap());

    let correct_signature = "896c8da285fca88cd938a039d7d4870a47808b94c73aad2dc6e346e4567b49514239ef93d782c7192f797b9f6f7096b944bca9ff245b6288a373875c8c8f090a";
    println!(" sig: {:?}", sig);
    assert_eq!(sig, correct_signature);
}

#[test]
fn create_and_verify_signature() {
    let acc = Account::from_signing_key(SIGNING_KEY_HEX).unwrap();
    let message = "testing create signature";
    let sig = acc.create_signature(message);
    assert_eq!(sig.len(), 128);
    let result = Account::verify_signature(&sig, message, &ACCOUNT_NUMBER_HEX);
    assert_eq!(result, true);
    // Testing with wrong message
    assert_eq!(
        Account::verify_signature(&sig, "testing create", &ACCOUNT_NUMBER_HEX),
        false
    );
    // Testing with wrong Account number
    assert_eq!(
        Account::verify_signature(&sig, "testing create", &Account::new().account_number()),
        false
    );
}

#[test]
fn create_block_message() {
    let acc = Account::from_signing_key(
        "4b3e69add153435a30c03f6ba4576cedeacfd9d362272a39863f0f3e37eda72c",
    )
    .unwrap();

    let tx = Transaction {
        amount: 1,
        fee: None,
        recipient: "000000000000000000000000000000000000000000000000000000000000dead",
        memo: None,
    };

    let node_fee = Transaction {
        amount: 1,
        fee: Some(NodeType::BANK),
        recipient: "29865762fae7d26e51f6465b3fea436d513478cfb8aa068e88a927e887cdc5fc",
        memo: None,
    };

    let pv_fee = Transaction {
        amount: 1,
        fee: Some(NodeType::PRIMARY_VALIDATOR),
        recipient: "ec8f6734272e8d9d5ea995479dd6d173424be38b313a3069d5fa62e7038a08e9",
        memo: None,
    };

    let txs = vec![&tx, &node_fee, &pv_fee];
    let balance_key =
        "1329d3a5d4a5ec2382dc539e03f30c3760e01932834a23522d3de0393b63f224".to_string();

    let block_data = BlockType::coin_transfer(balance_key, txs);
    let block_message = acc.create_block_message(&block_data);

    let serialized_data = serde_json::to_string(&block_data).unwrap();

    println!("block_message: {:?}\n\n", block_message);

    assert_eq!(
        Account::verify_signature(
            &block_message.signature,
            &serialized_data,
            acc.account_number()
        ),
        true
    );
}
