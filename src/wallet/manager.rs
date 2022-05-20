use super::keyfile::KeyFile;
use super::keystore::KeyStore;
use crate::error::Error;
use crate::global::znn_default_paths;
use std::fs::File;
use std::io::prelude::*;

use std::path::PathBuf;

pub struct KeyStoreManager {
    pub wallet_path: PathBuf,
    pub keystore_in_use: KeyStore,
}

impl KeyStoreManager {
    pub fn new(wallet_path: PathBuf, keystore_in_use: KeyStore) -> Self {
        Self {
            wallet_path,
            keystore_in_use,
        }
    }

    pub async fn save_keystore(store: &KeyStore, password: &str, name: &str) -> Result<(), Error> {
        let encrypted_kf: KeyFile = KeyFile::encrypt(store.clone(), password.to_string()).await?;
        // TODO: create real path
        let path = znn_default_paths()?.wallet.join(name);
        let mut file = File::create(&path)?;
        let serialized = serde_json::to_string(&encrypted_kf).unwrap();
        Ok(file.write_all(serialized.as_bytes())?)
    }
}
