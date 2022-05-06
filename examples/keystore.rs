extern crate znn_sdk_rust as znn;

use znn::wallet::{keypair::KeyPair, keystore::KeyStore};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    keystore_example().await;
    // api_example().await;
    // keyfile_example().await;
}

async fn keystore_example() {
    // crypto - keystore
    let mnemonic = "route become dream access impulse price inform obtain engage ski believe awful absent pig thing vibrant possible exotic flee pepper marble rural fire fancy".to_string();
    let keystore = KeyStore::from_mnemonic(mnemonic).unwrap();

    println!("entropy: {}", hex::encode(&keystore.entropy));
    println!("mnemonic: {}", &keystore.mnemonic);
    println!("seed: {}", hex::encode(&keystore.seed));

    // crypto - keypair
    let keypair: KeyPair = keystore.get_keypair().unwrap();
    let secret_key: &Vec<u8> = keypair.get_secret_key();
    let public_key: &Vec<u8> = keypair.get_public_key();
    let address: &Vec<u8> = keypair.address().get_bytes();

    println!("secret key: {}", hex::encode(secret_key));
    println!("public key: {}", hex::encode(public_key));
    println!("address: {}", hex::encode(address));

    println!("secret key: {:?}", secret_key);
    println!("public key: {:?}", public_key);
    println!("address: {:?}", address);
}
