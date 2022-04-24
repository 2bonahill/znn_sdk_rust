use pretty_assertions::assert_eq;
use znn_sdk_rust::wallet::{keypair::KeyPair, keystore::KeyStore};

// importing common module.
mod common;

#[test]
fn test_wallet() {
    common::setup();
    // crypto - keystore
    let mnemonic = "route become dream access impulse price inform obtain engage ski believe awful absent pig thing vibrant possible exotic flee pepper marble rural fire fancy".to_string();
    let keystore = KeyStore::from_mnemonic(mnemonic);

    assert_eq!(
        hex::encode(&keystore.entropy),
        "bc827d0a00a72354dce4c44a59485288500b49382f9ba88a016351787b7b15ca",
    );
    assert_eq!(
        &keystore.mnemonic,
        "route become dream access impulse price inform obtain engage ski believe awful absent pig thing vibrant possible exotic flee pepper marble rural fire fancy",
    );
    assert_eq!(
        hex::encode(&keystore.seed),
        "19f1d107d49f42ebc14d46b51001c731569f142590fdd20167ddeedbb201516731ad5ac9b58d3a1c9c09debfe62538379461e4ea9f038124c428784fecc645b7",
    );

    let keypair: KeyPair = keystore.get_keypair();
    let secret_key: &Vec<u8> = keypair.get_secret_key();
    let public_key: &Vec<u8> = keypair.get_public_key();
    let address: &Vec<u8> = keypair.get_address();

    assert_eq!(
        secret_key,
        &[
            214, 176, 31, 150, 181, 102, 215, 223, 155, 91, 83, 177, 151, 30, 75, 174, 183, 76,
            198, 65, 103, 169, 132, 63, 130, 208, 75, 33, 148, 202, 72, 99
        ]
    );

    assert_eq!(
        public_key,
        &[
            62, 19, 215, 35, 141, 14, 118, 138, 86, 125, 206, 132, 181, 73, 21, 242, 50, 63, 45,
            205, 14, 249, 167, 22, 217, 198, 26, 190, 214, 49, 186, 16
        ]
    );

    assert_eq!(
        address,
        &[0, 37, 55, 74, 65, 159, 50, 115, 111, 97, 236, 197, 172, 64, 89, 210, 241, 181, 136, 77]
    );

    // println!("secret key: {}", hex::encode(secret_key));
    // println!("public key: {}", hex::encode(public_key));
    // println!("address: {}", hex::encode(address));
}
