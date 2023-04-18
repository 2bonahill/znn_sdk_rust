# Zenon Rust SDK

Reference implementation for the Zenon SDK for Rust. Compatible with the Zenon Alphanet - Network of Momentum Phase 0. It provides a simple integration with any Rust based projects.

## Zenon Accelerator-Z 

The project proposal was accepted in the Accelerator-Z program.

## Setup

To start using this library, edit the `Cargo.toml` by adding the following lines: 

```yaml
[dependencies]
  znn_sdk_rust = { path = "path_to_library" }
```

> Notice: `znn_sdk_rust` requires Rust version `>=1.59.0`

## Examples

You can find all [examples](./examples) in the examples folder.

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
  use znn_sdk_rust as znn;
  ...
  
  let znn = Zenon::init("ws://public.deeZNNodez.com:35998")
        .await
        .unwrap();
    println!("is the client connected?: {}", znn.client.is_connected());
    assert_eq!(znn.client.is_connected(), true);

    let pil: PillarInfoList = znn.embedded.pillar.get_all(1, 10).await.unwrap();
    println!("Number of pillars: {}", pil.count);

    let a = Address::parse("z1qq0hffeyj0htmnr4gc6grd8zmqfvwzgrydt402").unwrap();
    let ai: AccountInfo = znn.ledger.get_account_info_by_address(a).await.unwrap();
    dbg!("Account info: {}", ai);
```

## Contributing

Please contact us for more details.
