use super::keystore::KeyStore;
use crate::model::primitives::address::Address;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use anyhow::Result;
use argon2::{self, Config, ThreadMode, Variant, Version};
use rand::Rng;
use std::time::SystemTime;

#[derive(Debug, Default)]
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

    pub async fn encrypt(store: KeyStore, password: String) -> Result<KeyFile> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
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
        stored._encrypt_entropy(store, password);
        Ok(stored)
    }

    fn _encrypt_entropy(&mut self, store: KeyStore, password: String) {
        let salt = rand::thread_rng().gen::<[u8; 16]>();

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
        let hash = argon2::hash_raw(password.as_bytes(), &salt, &config).unwrap();

        // encrypt entropy using the argon2 pwd hash
        let key = Key::from_slice(&hash);
        let cipher = Aes256Gcm::new(key);
        let nonce_random = rand::thread_rng().gen::<[u8; 12]>();
        let nonce = GenericArray::from_slice(&nonce_random);
        let encrypted = cipher
            .encrypt(nonce, b"plaintext message".as_ref())
            .expect("encryption failure!"); // NOTE: handle this error to avoid panics!
        self.crypto.cipher_data = encrypted;
        self.crypto.nonce = nonce.to_vec();
        self.crypto.argon2_params.salt = salt.to_vec();
    }
}

#[derive(Debug, Default)]
pub struct _Crypto {
    argon2_params: _Argon2Params,
    cipher_data: Vec<u8>,
    cipher_name: String,
    kdf: String,
    nonce: Vec<u8>,
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
    #[tokio::test]
    async fn test_keystore_encryption() -> Result<()> {
        const ENTROPY: [u8; 32] = [
            188, 130, 125, 10, 0, 167, 35, 84, 220, 228, 196, 74, 89, 72, 82, 136, 80, 11, 73, 56,
            47, 155, 168, 138, 1, 99, 81, 120, 123, 123, 21, 202,
        ];
        let ks: KeyStore = KeyStore::from_entropy(ENTROPY.to_vec()).unwrap();
        dbg!(&ks);
        let kf = KeyFile::encrypt(ks, "pwd".to_string()).await?;
        dbg!(&kf);
        assert_eq!(1, 1);
        Ok(())
    }
}
