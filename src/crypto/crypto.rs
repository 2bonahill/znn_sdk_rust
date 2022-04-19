use ed25519_dalek_bip32::DerivationPath;
use ed25519_dalek_bip32::ExtendedSecretKey;
use sha3::{Digest, Sha3_256, Sha3_512};

pub fn derive_key(path: String, seed: &Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let dp: DerivationPath = parse_derivation_path(&path);
    let extended_secret_key: ExtendedSecretKey = ExtendedSecretKey::from_seed(seed)
        .expect("Unable to generate secret key from seed.")
        .derive(&dp)
        .expect("Failed path derivation");

    let secret_key: [u8; 32] = extended_secret_key.secret_key.to_bytes();
    let public_key: [u8; 32] = extended_secret_key.public_key().to_bytes();
    let address: [u8; 32] = derive_address_from_public_key(&public_key);

    (secret_key.into(), public_key.into(), address.into())
}

fn parse_derivation_path(path: &str) -> DerivationPath {
    path.parse::<DerivationPath>()
        .expect("Failed parsing derivation path")
}

fn derive_address_from_public_key(public_key: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(public_key);
    hasher.finalize().into()
}

fn test_derive_address_from_public_key(public_key: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha3_512::new();
    // hasher.update(public_key);
    // hasher.finalize().into()
    todo!()
}
