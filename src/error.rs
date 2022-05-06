extern crate aes_gcm;
use std::fmt::Debug;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed encrypting the keystore: {0}")]
    KeyStoreEncrpytionError(#[from] aes_gcm::Error),

    #[error("Failed to derive key from seed and path: {0}")]
    KeyDerivationError(#[from] ed25519_dalek_bip32::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
