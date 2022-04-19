use ed25519_dalek_bip32::DerivationPath;
use ed25519_dalek_bip32::ExtendedSecretKey;
use sha3::{Digest, Sha3_256};

pub fn derive_key(path: String, seed: &Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let dp: DerivationPath = parse_derivation_path(&path);
    let extended_secret_key: ExtendedSecretKey = ExtendedSecretKey::from_seed(seed)
        .expect("Unable to generate secret key from seed.")
        .derive(&dp)
        .expect("Failed path derivation");

    let secret_key: [u8; 32] = extended_secret_key.secret_key.to_bytes();
    let public_key: [u8; 32] = extended_secret_key.public_key().to_bytes();
    let address: Vec<u8> = derive_address_from_public_key(&public_key);

    (secret_key.into(), public_key.into(), address.into())
}

fn parse_derivation_path(path: &str) -> DerivationPath {
    path.parse::<DerivationPath>()
        .expect("Failed parsing derivation path")
}

fn derive_address_from_public_key(public_key: &[u8; 32]) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(public_key);
    let hash: [u8; 32] = hasher.finalize().into();
    let mut hash_vector: Vec<u8> = hash.to_vec();
    hash_vector.insert(0, 0u8);
    hash_vector.truncate(20);
    hash_vector
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::crypto::crypto;

    const PATH: &str = "m/44'/73404'/0'";
    const SEED: [u8; 64] = [
        25, 241, 209, 7, 212, 159, 66, 235, 193, 77, 70, 181, 16, 1, 199, 49, 86, 159, 20, 37, 144,
        253, 210, 1, 103, 221, 238, 219, 178, 1, 81, 103, 49, 173, 90, 201, 181, 141, 58, 28, 156,
        9, 222, 191, 230, 37, 56, 55, 148, 97, 228, 234, 159, 3, 129, 36, 196, 40, 120, 79, 236,
        198, 69, 183,
    ];
    const ENTROPY: [u8; 32] = [
        188, 130, 125, 10, 0, 167, 35, 84, 220, 228, 196, 74, 89, 72, 82, 136, 80, 11, 73, 56, 47,
        155, 168, 138, 1, 99, 81, 120, 123, 123, 21, 202,
    ];

    #[test]
    fn test_derive_key() {}
}
