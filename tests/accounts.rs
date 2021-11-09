use tnb_rs::*;

const SIGNING_KEY_HEX: &str = "8cf08eb96b00b5a4df86a750bb7ae595a9dbbe91fc091463bfb3d950d5dac467";
const ACCOUNT_NUMBER_HEX: &str = "e6ba479bc9098608d4bb756ff80093ffa1c2200c3c282b90e9d2f5e1f7adab41";

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
fn serialize_block() {
    let block_data: BlockData = BlockData::CoinTransfer {
        balance_key: String::from(
            "72fe3f3cc0b70a7f75d21e14b092ea805fc109eb7137e431fe8a94b2df3dc4a6",
        ),
        txs: vec![
            &Transaction {
                amount: 1,
                recipient: "06e51367ffdb5e3e3c31118596e0956a48b1ffde327974d39ce1c3d3685e30ab",
                fee: None,
                memo: Some("AEz"),
            },
            &Transaction {
                amount: 1,
                recipient: "29865762fae7d26e51f6465b3fea436d513478cfb8aa068e88a927e887cdc5fc",
                fee: Some(NodeType::BANK),
                memo: None,
            },
            &Transaction {
                amount: 1,
                recipient: "ec8f6734272e8d9d5ea995479dd6d173424be38b313a3069d5fa62e7038a08e9",
                fee: Some(NodeType::PRIMARY_VALIDATOR),
                memo: None,
            },
        ],
    };
    let block_message = BlockMessage{
        account_number: "72fe3f3cc0b70a7f75d21e14b092ea805fc109eb7137e431fe8a94b2df3dc4a6",
        message: &block_data,
        signature: "ee5a2f2a2f5261c1b633e08dd61182fd0db5604c853ebd8498f6f28ce8e2ccbbc38093918610ea88a7ad47c7f3192ed955d9d1529e7e390013e43f25a5915c0f".to_string(),
    };
    let test_block = "{\"account_number\":\"72fe3f3cc0b70a7f75d21e14b092ea805fc109eb7137e431fe8a94b2df3dc4a6\",\"message\":{\"balance_key\":\"72fe3f3cc0b70a7f75d21e14b092ea805fc109eb7137e431fe8a94b2df3dc4a6\",\"txs\":[{\"amount\":1,\"recipient\":\"06e51367ffdb5e3e3c31118596e0956a48b1ffde327974d39ce1c3d3685e30ab\",\"memo\":\"AEz\"},{\"amount\":1,\"recipient\":\"29865762fae7d26e51f6465b3fea436d513478cfb8aa068e88a927e887cdc5fc\",\"fee\":\"BANK\"},{\"amount\":1,\"recipient\":\"ec8f6734272e8d9d5ea995479dd6d173424be38b313a3069d5fa62e7038a08e9\",\"fee\":\"PRIMARY_VALIDATOR\"}]},\"signature\":\"ee5a2f2a2f5261c1b633e08dd61182fd0db5604c853ebd8498f6f28ce8e2ccbbc38093918610ea88a7ad47c7f3192ed955d9d1529e7e390013e43f25a5915c0f\"}";
    assert_eq!(serde_json::to_string(&block_message).unwrap(), test_block);
}

#[test]
fn create_block_message() {
    let acc = Account::from_signing_key(SIGNING_KEY_HEX).unwrap();

    let tx = Transaction {
        amount: 1000,
        recipient: acc.account_number(),
        fee: None,
        memo: None,
    };

    let data = BlockData::CoinTransfer {
        balance_key: Account::new().account_number().to_string(),
        txs: vec![&tx],
    };

    let block = acc.create_block_message(&data);
    let serialized_data = serde_json::to_string(&data).unwrap();
    assert_eq!(
        Account::verify_signature(&block.signature, &serialized_data, acc.account_number()),
        true
    );
}
