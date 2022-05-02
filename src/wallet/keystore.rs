use super::keypair::KeyPair;
use crate::{crypto::crypto, model::primitives::address::Address};
use anyhow::Result;
use bip39::*;

#[derive(Debug, Default)]
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

    fn set_mnemonic(&mut self, mnemonic: String) -> Result<()> {
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
    use crate::wallet::keystore::KeyStore;
    use anyhow::Result;

    const MNEMONIC: &str = "route become dream access impulse price inform obtain engage ski believe awful absent pig thing vibrant possible exotic flee pepper marble rural fire fancy";
    const SEED: [u8; 64] = [
        25, 241, 209, 7, 212, 159, 66, 235, 193, 77, 70, 181, 16, 1, 199, 49, 86, 159, 20, 37, 144,
        253, 210, 1, 103, 221, 238, 219, 178, 1, 81, 103, 49, 173, 90, 201, 181, 141, 58, 28, 156,
        9, 222, 191, 230, 37, 56, 55, 148, 97, 228, 234, 159, 3, 129, 36, 196, 40, 120, 79, 236,
        198, 69, 183,
    ];
    const ENTROPY: [u8; 32] = [
        188, 130, 125, 10, 0, 167, 35, 84, 220, 228, 196, 74, 89, 72, 82, 136, 80, 11, 73, 56, 47,
        155, 168, 138, 1, 99, 81, 120, 123, 123, 21, 202,
    ];

    #[test]
    fn test_keystore_from_mnemonic() -> Result<()> {
        let key_store = KeyStore::from_mnemonic(MNEMONIC.to_string())?;
        assert_eq!(key_store.mnemonic, MNEMONIC);
        assert_eq!(key_store.seed, SEED);
        assert_eq!(key_store.entropy, ENTROPY);
        Ok(())
    }
}
