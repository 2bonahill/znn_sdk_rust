# Zenon Rust SDK

Reference implementation for the Zenon SDK for Rust. Compatible with the Zenon Alphanet - Network of Momentum Phase 0. It provides a simple integration with any Rust based projects.

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
    use znn_sdk_rust::wallet::{keypair::KeyPair, keystore::KeyStore};

    ...
    
    let mnemonic = "route become dream access impulse price inform obtain engage ski believe awful absent pig thing vibrant possible exotic flee pepper marble rural fire fancy".to_string();

    let keystore = KeyStore::from_mnemonic(mnemonic);

    println!("entropy: {}", hex::encode(&keystore.entropy));
    println!("mnemonic: {}", &keystore.mnemonic);
    println!("seed: {}", hex::encode(&keystore.seed));

    let keypair: KeyPair = keystore.get_keypair();
    let secret_key: &Vec<u8> = keypair.get_secret_key();
    let public_key: &Vec<u8> = keypair.get_public_key();

    println!("secret key: {}", hex::encode(secret_key));
    println!("public key: {}", hex::encode(public_key));
```

## Contributing

Please contact us for more details.