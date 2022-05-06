use anyhow::Result;
use ed25519_dalek_bip32::DerivationPath;
use ed25519_dalek_bip32::ExtendedSecretKey;
use sha3::{Digest, Sha3_256};

use crate::error::Error;

pub fn derive_key(path: String, seed: &Vec<u8>) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), Error> {
    let dp: DerivationPath = parse_derivation_path(&path)?;
    let extended_secret_key: ExtendedSecretKey = ExtendedSecretKey::from_seed(seed)?.derive(&dp)?;

    let secret_key: [u8; 32] = extended_secret_key.secret_key.to_bytes();
    let public_key: [u8; 32] = extended_secret_key.public_key().to_bytes();
    let address: Vec<u8> = derive_address_bytes_from_public_key(&public_key);

    Ok((secret_key.into(), public_key.into(), address.into()))
}

fn parse_derivation_path(path: &str) -> Result<DerivationPath> {
    Ok(path.parse::<DerivationPath>()?)
}

pub fn derive_address_bytes_from_public_key(public_key: &[u8; 32]) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(public_key);
    let mut hash: Vec<u8> = hasher.finalize().to_vec();
    hash.insert(0, 0u8);
    hash.truncate(20);
    hash
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use anyhow::Ok;
    use anyhow::Result;

    use crate::crypto::crypto;

    const PATH: &str = "m/44'/73404'/0'";
    const SEED: [u8; 64] = [
        25, 241, 209, 7, 212, 159, 66, 235, 193, 77, 70, 181, 16, 1, 199, 49, 86, 159, 20, 37, 144,
        253, 210, 1, 103, 221, 238, 219, 178, 1, 81, 103, 49, 173, 90, 201, 181, 141, 58, 28, 156,
        9, 222, 191, 230, 37, 56, 55, 148, 97, 228, 234, 159, 3, 129, 36, 196, 40, 120, 79, 236,
        198, 69, 183,
    ];
    const SECRET_KEY: [u8; 32] = [
        214, 176, 31, 150, 181, 102, 215, 223, 155, 91, 83, 177, 151, 30, 75, 174, 183, 76, 198,
        65, 103, 169, 132, 63, 130, 208, 75, 33, 148, 202, 72, 99,
    ];
    const PUBLIC_KEY: [u8; 32] = [
        62, 19, 215, 35, 141, 14, 118, 138, 86, 125, 206, 132, 181, 73, 21, 242, 50, 63, 45, 205,
        14, 249, 167, 22, 217, 198, 26, 190, 214, 49, 186, 16,
    ];
    const ADDRESS: [u8; 20] = [
        0, 37, 55, 74, 65, 159, 50, 115, 111, 97, 236, 197, 172, 64, 89, 210, 241, 181, 136, 77,
    ];

    #[test]
    fn test_derive_key() -> Result<()> {
        let (secret_key, public_key, address) =
            crypto::derive_key(PATH.to_string(), &SEED.to_vec())?;
        assert_eq!(secret_key, SECRET_KEY);
        assert_eq!(public_key, PUBLIC_KEY);
        assert_eq!(address, ADDRESS);
        Ok(())
    }
}
