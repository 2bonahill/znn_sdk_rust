extern crate aes_gcm;
use std::fmt::Debug;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed encrypting the keystore: '{0}'")]
    KeyStoreEncrpytionError(#[from] aes_gcm::Error),

    #[error("Failed to derive key from seed and path: '{0}'")]
    KeyDerivationError(#[from] ed25519_dalek_bip32::Error),

    #[error("Failed to parse derivation path: '{0}'")]
    DerivationPathParseError(
        #[from] ed25519_dalek_bip32::derivation_path::DerivationPathParseError,
    ),

    #[error("Failed to get public key from secret key: '{0}'")]
    DerivePublicKeyFromSecretKeyError(#[from] ed25519_dalek::ed25519::Error),

    #[error("Failed to save keystore : '{0}'")]
    SavingKeyStoreError(#[from] std::io::Error),

    #[error("Failed to get default path '{0}'")]
    FailedGettingPath(String),

    #[error("Error from websocket rpc client : '{0}'")]
    WebSocketError(#[from] jsonrpsee_core::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
