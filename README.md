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
    znn_sdk_rust::hi_from_zenon();
    let mnemonic = "route become dream access impulse price inform obtain engage ski believe awful absent pig thing vibrant possible exotic flee pepper marble rural fire fancy".to_string();

    let keystore = KeyStore::from_mnemonic(mnemonic);

    println!("entropy: {}", hex::encode(&keystore.entropy));
    println!("mnemonic: {}", &keystore.mnemonic);
    println!("seed: {}", hex::encode(&keystore.seed));

    let keypair: KeyPair = keystore.get_keypair();
    let private_key = keypair.get_private_key();
    let public_key = keypair.get_public_key();

    println!("secret key: {}", hex::encode(private_key));
    println!("public key: {}", hex::encode(public_key));
```


## Contributing

Please contact us for more details.

## License

The MIT License (MIT). Please check LICENSE for more information.