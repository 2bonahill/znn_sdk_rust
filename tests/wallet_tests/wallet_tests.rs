use anyhow::Result;
use pretty_assertions::assert_eq;
use znn_sdk_rust::wallet::{keyfile::KeyFile, keypair::KeyPair, keystore::KeyStore};

use crate::wallet_tests::test_data;

#[test]
pub fn test_keystore_from_mnemonic() -> Result<()> {
    let keystore = KeyStore::from_mnemonic(test_data::MNEMONIC.to_string())?;

    assert_eq!(keystore.entropy, test_data::ENTROPY);
    assert_eq!(keystore.mnemonic, test_data::MNEMONIC);
    assert_eq!(keystore.seed, test_data::SEED,);

    let keypair: KeyPair = keystore.get_keypair()?;
    let secret_key: &Vec<u8> = keypair.get_secret_key();
    let public_key: &Vec<u8> = keypair.get_public_key();
    let address: &Vec<u8> = keypair.address().get_bytes();

    assert_eq!(secret_key, &test_data::SECRET_KEKY);
    assert_eq!(public_key, &test_data::PUBLIC_KEY);
    assert_eq!(address, &test_data::ADDRESS);

    Ok(())
}

#[tokio::test]
pub async fn test_keyfile() -> Result<()> {
    let keystore = KeyStore::from_mnemonic(test_data::MNEMONIC.to_string())?;
    let encrypted_kf: KeyFile =
        KeyFile::encrypt(keystore.clone(), "my password".to_string()).await?;

    let decrypted_ks: KeyStore = KeyFile::decrypt(encrypted_kf, "my password".to_string()).await?;
    assert_eq!(&keystore.entropy, &decrypted_ks.entropy);
    assert_eq!(&keystore.seed, &decrypted_ks.seed);
    assert_eq!(&keystore.mnemonic, &decrypted_ks.mnemonic);

    Ok(())
}
