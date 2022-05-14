use super::keystore::KeyStore;
use crate::error::Error;
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

    #[allow(unused_variables)]
    pub fn save_keystore(store: KeyStore, password: &str, name: &str) -> Result<(), Error> {
        todo!();
        // Ok(())
    }
}
