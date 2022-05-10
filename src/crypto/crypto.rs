use ed25519_dalek::ExpandedSecretKey;
use ed25519_dalek::PublicKey;
use ed25519_dalek::SecretKey;
use ed25519_dalek::Signature;
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

pub fn get_public_key(secret_key: &[u8; 32]) -> Result<[u8; 32], Error> {
    // let x = ed25519_dalek_bip32::ExtendedSecretKey::derive(&self, path)
    let secret_key: SecretKey = SecretKey::from_bytes(secret_key)?;
    let public_key: PublicKey = PublicKey::from(&secret_key);
    Ok(public_key.to_bytes())
    // Ok([0u8; 32])
}

pub fn derive_address_bytes_from_public_key(public_key: &[u8; 32]) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(public_key);
    let mut hash: Vec<u8> = hasher.finalize().to_vec();
    hash.insert(0, 0u8);
    hash.truncate(20);
    hash
}

pub fn sign(message: Vec<u8>, secret_key: Vec<u8>, public_key: Vec<u8>) -> Result<Vec<u8>, Error> {
    let key_pair = ExpandedSecretKey::from(&SecretKey::from_bytes(&secret_key)?);
    let signed = key_pair.sign(&message, &PublicKey::from_bytes(&public_key)?);
    Ok(signed.to_bytes().to_vec())
}

pub fn verify(signature: Vec<u8>, message: Vec<u8>, public_key: Vec<u8>) -> Result<(), Error> {
    let pk: PublicKey = PublicKey::from_bytes(&public_key)?;
    let s: Signature = Signature::from_bytes(&signature)?;
    Ok(pk.verify_strict(&message, &s)?)
}

fn parse_derivation_path(path: &str) -> Result<DerivationPath, Error> {
    Ok(path.parse::<DerivationPath>()?)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::derive_address_bytes_from_public_key;
    use super::get_public_key;
    use super::parse_derivation_path;
    use crate::crypto::crypto;
    use crate::crypto::utils::unit_test_data::{ADDRESS, PATH, PUBLIC_KEY, SECRET_KEY, SEED};
    use crate::error::Error;

    #[test]
    fn test_derive_key() -> Result<(), Error> {
        let (secret_key, public_key, address) =
            crypto::derive_key(PATH.to_string(), &SEED.to_vec())?;
        assert_eq!(secret_key, SECRET_KEY);
        assert_eq!(public_key, PUBLIC_KEY);
        assert_eq!(address, ADDRESS);
        Ok(())
    }

    #[test]
    fn test_derive_address_bytes_from_public_key() -> Result<(), Error> {
        let hash: Vec<u8> = derive_address_bytes_from_public_key(&PUBLIC_KEY);
        assert_eq!(hash[0], 0);
        assert_eq!(hash.len(), 20);
        Ok(())
    }

    #[test]
    fn test_parse_derivation_path() -> Result<(), Error> {
        let x = parse_derivation_path("invalid path");
        assert!(x.is_err());
        Ok(())
    }

    #[test]
    fn test_get_public_key() -> Result<(), Error> {
        let secret_key = get_public_key(&SECRET_KEY)?;
        assert_eq!(secret_key, PUBLIC_KEY);
        Ok(())
    }
}
