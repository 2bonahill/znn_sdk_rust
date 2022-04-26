# Zenon Rust SDK

Reference implementation for the Zenon SDK for Rust. Compatible with the Zenon Alphanet - Network of Momentum Phase 0. It provides a simple integration with any Rust based projects.

## Zenon Accelerator-Z 

The project proposal can be found here: https://github.com/2bonahill/znn_sdk_rust/blob/main/AZ.md

## Setup

To start using this library, edit the `Cargo.toml` by adding the following lines:

```yaml
[dependencies]
  znn_sdk_rust = { path = "path_to_library" }
```

> Notice: `znn_sdk_rust` requires Rust version `>=1.59.0`

## Examples

### Example 1 - Key handling
```rust
  use znn_sdk_rust as znn;
  use znn::client::websocket::WsClient;
  use znn::wallet::{keypair::KeyPair, keystore::KeyStore};

  ...
  
  // say hello
  znn::hi_from_zenon();

  // crypto - keystore
  let mnemonic = "route become dream access impulse price inform obtain engage ski believe awful absent pig thing vibrant possible exotic flee pepper marble rural fire fancy".to_string();
  let keystore = KeyStore::from_mnemonic(mnemonic);

  println!("entropy: {}", hex::encode(&keystore.entropy));
  println!("mnemonic: {}", &keystore.mnemonic);
  println!("seed: {}", hex::encode(&keystore.seed));

  // crypto - keypair
  let keypair: KeyPair = keystore.get_keypair();
  let secret_key: &Vec<u8> = keypair.get_secret_key();
  let public_key: &Vec<u8> = keypair.get_public_key();
  let address: &Vec<u8> = keypair.get_address();

  println!("secret key: {}", hex::encode(secret_key));
  println!("public key: {}", hex::encode(public_key));
  println!("address: {}", hex::encode(address));

  println!("secret key: {:?}", secret_key);
  println!("public key: {:?}", public_key);
  println!("address: {:?}", address);
```

### Example 2 - APIs
```rust
  use znn::client::websocket::WsClient;
  use znn::model::embedded::pillar::PillarInfoList;
  use znn::model::nom::account_info::AccountInfo;
  use znn::model::primitives::address::Address;
  use znn_sdk_rust as znn;

  ...
  
  let client: WsClient = WsClient::initialize("ws://nodes.zenon.place:35998")
    .await
    .unwrap();
    println!("is the client connected?: {}", client.is_connected());
    assert_eq!(client.is_connected(), true);

    let pil: PillarInfoList = znn::api::embedded::Pillar::get_all(&client, 1, 10)
      .await
      .unwrap();
    println!("Number of pillars: {}", pil.count);

    let a = Address {
        address: "z1qq0hffeyj0htmnr4gc6grd8zmqfvwzgrydt402".to_string(),
    };
    let ai: AccountInfo = znn::api::Ledger::get_account_info_by_address(&client, a)
      .await
      .unwrap();
    dbg!("Account info: {}", ai);
```

## Contributing

Please contact us for more details.