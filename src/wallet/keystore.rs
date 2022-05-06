use super::keypair::KeyPair;
use crate::{crypto::crypto, error::Error, model::primitives::address::Address};
use anyhow::Result;
use bip39::*;

#[derive(Debug, Default, Clone)]
pub struct KeyStore {
    pub entropy: Vec<u8>,
    pub mnemonic: String,
    pub seed: Vec<u8>,
}

impl KeyStore {
    pub fn from_mnemonic(mnemonic: String) -> Result<Self> {
        let mut s: KeyStore = KeyStore::default();
        s.set_mnemonic(mnemonic)?;
        Ok(s)
    }

    pub fn from_seed(seed: Vec<u8>) -> Self {
        let mut s: KeyStore = KeyStore::default();
        s.set_seed(seed);
        s
    }

    pub fn from_entropy(entropy: Vec<u8>) -> Result<Self> {
        let mut s: KeyStore = KeyStore::default();
        s.set_entropy(entropy)?;
        Ok(s)
    }

    fn set_mnemonic(&mut self, mnemonic: String) -> Result<(), Error> {
        let mn = Mnemonic::from_phrase(&mnemonic, Language::English)?;
        self.mnemonic = mnemonic;

        self.entropy = mn.entropy().into();

        let seed = Seed::new(&mn, "");
        self.seed = seed.as_bytes().into();
        Ok(())
    }

    fn set_seed(&mut self, seed: Vec<u8>) {
        self.seed = seed;
    }

    fn set_entropy(&mut self, entropy: Vec<u8>) -> Result<()> {
        let mnemonic = Mnemonic::from_entropy(&entropy, Language::English)?.into_phrase();
        self.set_mnemonic(mnemonic)?;
        Ok(())
    }

    pub fn get_keypair(&self) -> Result<KeyPair> {
        // BIP44 https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki
        // m / purpose' / coin_type' / account' / change / address_index
        let (secret_key, public_key, address_core) =
            crypto::derive_key("m/44'/73404'/0'".to_string(), &self.seed)?;
        let a: Address = Address::new("z".to_string(), address_core);
        Ok(KeyPair::new(secret_key, public_key, a))
    }
}

#[cfg(test)]
mod tests {
    use crate::wallet::{
        keystore::KeyStore,
        utils::unit_test_data::{ENTROPY, MNEMONIC, SEED},
    };
    use anyhow::Result;

    #[test]
    fn test_keystore_from_mnemonic() -> Result<()> {
        let key_store = KeyStore::from_mnemonic(MNEMONIC.to_string())?;
        assert_eq!(key_store.mnemonic, MNEMONIC);
        assert_eq!(key_store.seed, SEED);
        assert_eq!(key_store.entropy, ENTROPY);
        Ok(())
    }
}
