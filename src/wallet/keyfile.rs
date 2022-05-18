use super::keystore::KeyStore;
use crate::error::Error;
use crate::model::primitives::address::Address;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key}; // Or `Aes128Gcm`
use anyhow::Result;
use argon2::{self, Config, ThreadMode, Variant, Version};
use jsonrpsee_core::Serialize;
use rand::Rng;
use serde::ser::{SerializeSeq, SerializeStruct};
use serde::Serializer;
use std::time::SystemTime;

#[derive(Debug, Default, Serialize)]
pub struct KeyFile {
    pub base_address: Address,
    pub crypto: _Crypto,
    pub timestamp: u64,
    pub version: u64,
}

impl KeyFile {
    pub fn new(base_address: Address, crypto: _Crypto, timestamp: u64, version: u64) -> Self {
        Self {
            base_address,
            crypto,
            timestamp,
            version,
        }
    }

    pub async fn encrypt(store: KeyStore, password: String) -> Result<KeyFile, Error> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let base_address = store.get_keypair()?.address().clone();
        let version = 1;

        let _crypto: _Crypto = _Crypto::new(
            _Argon2Params::new(vec![0u8]),
            vec![0u8],
            "aes-256-gcm".to_string(),
            "argon2.IDKey".to_string(),
            vec![0u8],
        );

        let mut stored: KeyFile = KeyFile::new(base_address, _crypto, timestamp, version);
        stored.encrypt_entropy(store, password)?;
        Ok(stored)
    }

    fn encrypt_entropy(&mut self, store: KeyStore, password: String) -> Result<(), Error> {
        let config = Config {
            variant: Variant::Argon2id,
            version: Version::Version13,
            mem_cost: 64 * 1024,
            time_cost: 1,
            lanes: 4,
            thread_mode: ThreadMode::Parallel,
            secret: &[],
            ad: &[],
            hash_length: 32,
        };
        let salt = rand::thread_rng().gen::<[u8; 16]>();
        let hash = argon2::hash_raw(password.as_bytes(), &salt, &config).unwrap();

        // generate key using the argon2 pwd hash
        let key = Key::from_slice(&hash);
        let cipher = Aes256Gcm::new(key);
        let nonce_random = rand::thread_rng().gen::<[u8; 12]>();
        let nonce = GenericArray::from_slice(&nonce_random);
        let encrypted = cipher.encrypt(nonce, store.entropy.as_ref())?;
        self.crypto.cipher_data = encrypted;
        self.crypto.nonce = nonce.to_vec();
        self.crypto.argon2_params.salt = salt.to_vec();
        Ok(())
    }

    pub async fn decrypt(keyfile: KeyFile, password: String) -> Result<KeyStore, Error> {
        let config = Config {
            variant: Variant::Argon2id,
            version: Version::Version13,
            mem_cost: 64 * 1024,
            time_cost: 1,
            lanes: 4,
            thread_mode: ThreadMode::Parallel,
            secret: &[],
            ad: &[],
            hash_length: 32,
        };
        let salt = keyfile.crypto.argon2_params.salt;
        let hash = argon2::hash_raw(password.as_bytes(), &salt, &config).unwrap();

        // generate key using the argon2 pwd hash
        let key = Key::from_slice(&hash);
        let cipher = Aes256Gcm::new(key);
        let nonce = GenericArray::from_slice(keyfile.crypto.nonce.as_slice());

        let ciphertext: &[u8] = &keyfile.crypto.cipher_data[..];
        let plaintext: Vec<u8> = cipher.decrypt(nonce, ciphertext)?;
        let ks = KeyStore::from_entropy(plaintext)?;
        Ok(ks)
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, Serialize)]
pub struct _Crypto {
    argon2_params: _Argon2Params,
    #[serde(serialize_with = "serialize_crypto_vec")]
    cipher_data: Vec<u8>,
    cipher_name: String,
    kdf: String,
    #[serde(serialize_with = "serialize_crypto_vec")]
    nonce: Vec<u8>,
}

pub fn serialize_crypto_vec<S>(v: &[u8], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex_string = hex::encode(&v);
    s.serialize_str(&hex_string)
}

impl _Crypto {
    pub fn new(
        argon2_params: _Argon2Params,
        cipher_data: Vec<u8>,
        cipher_name: String,
        kdf: String,
        nonce: Vec<u8>,
    ) -> Self {
        Self {
            argon2_params,
            cipher_data,
            cipher_name,
            kdf,
            nonce,
        }
    }
}

#[derive(Debug, Default)]
pub struct _Argon2Params {
    salt: Vec<u8>,
}

impl serde::ser::Serialize for _Argon2Params {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("argon2_params", 1)?;
        s.serialize_field("salt", &hex::encode(&self.salt))?;
        s.end()
    }
}

impl _Argon2Params {
    pub fn new(salt: Vec<u8>) -> Self {
        Self { salt }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::wallet::keyfile::KeyFile;
    use crate::wallet::keystore::KeyStore;
    use crate::wallet::utils::unit_test_data::ENTROPY;

    #[tokio::test]
    async fn test_keystore_encryption() -> Result<()> {
        let ks: KeyStore = KeyStore::from_entropy(ENTROPY.to_vec()).unwrap();
        let kf: KeyFile = KeyFile::encrypt(ks, "pwd".to_string()).await?;

        assert!(kf.crypto.argon2_params.salt.len() > 1);
        assert!(kf.crypto.cipher_data.len() > 1);
        assert!(kf.crypto.nonce.len() > 1);

        Ok(())
    }
}
