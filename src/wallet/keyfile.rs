use std::time::SystemTime;

use crate::model::primitives::address::Address;
use anyhow::Result;

use super::keystore::KeyStore;

pub struct KeyFile {
    pub base_address: Option<Address>,
    pub crypto: Option<_Crypto>,
    pub timestamp: Option<u64>,
    pub version: Option<u64>,
}

impl KeyFile {
    pub fn new(
        base_address: Option<Address>,
        crypto: Option<_Crypto>,
        timestamp: Option<u64>,
        version: Option<u64>,
    ) -> Self {
        Self {
            base_address,
            crypto,
            timestamp,
            version,
        }
    }

    pub async fn encrypt(store: KeyStore, password: String) -> Result<()> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        // let stored: KeyFile = KeyFile::new("", "crypto", "timestamp", "version");

        // stored._encrypt_entropy();
        Ok(())
    }
}

pub struct _Crypto {}
