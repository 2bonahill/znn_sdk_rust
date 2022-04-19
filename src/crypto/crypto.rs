use ed25519_dalek_bip32::ExtendedSecretKey;

// derive a secret key from a seed
pub fn derive_key(path: String, seed: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let extended_secret_key: ExtendedSecretKey =
        ExtendedSecretKey::from_seed(seed).expect("Unable to generate secret key from seed.");

    let secret_key: [u8; 32] = extended_secret_key.secret_key.to_bytes();
    let public_key: [u8; 32] = extended_secret_key.public_key().to_bytes();

    // println!("Secret key: {:?}", secret_key);
    // println!("public key: {:?}", public_key);

    // println!("secret key hex: {}", hex::encode(&secret_key));

    (secret_key.into(), public_key.into())
}
