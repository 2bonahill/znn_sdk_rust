use bip39::*;

#[derive(Debug, Default)]
pub struct KeyStore {
    pub entropy: Vec<u8>,
    pub mnemonic: String,
    pub seed: Vec<u8>,
}

#[allow(non_snake_case)]
impl KeyStore {
    pub fn fromMnemonic(mnemonic: String) -> Self {
        let mut s: KeyStore = KeyStore::default();
        s.setMnemonic(mnemonic);
        s
    }

    pub fn fromSeed(seed: Vec<u8>) -> Self {
        let mut s: KeyStore = KeyStore::default();
        s.setSeed(seed);
        s
    }

    pub fn fromEntropy(entropy: Vec<u8>) -> Self {
        let mut s: KeyStore = KeyStore::default();
        s.setEntropy(entropy);
        s
    }

    fn setMnemonic(&mut self, mnemonic: String) {
        let mn = Mnemonic::from_phrase(&mnemonic, Language::English).expect("Mnemonic not valid.");
        self.mnemonic = mnemonic;

        self.entropy = mn.entropy().into();

        let seed = Seed::new(&mn, "");
        self.seed = seed.as_bytes().into();
    }

    pub fn setSeed(&mut self, seed: Vec<u8>) {
        self.seed = seed;
    }

    pub fn setEntropy(&mut self, entropy: Vec<u8>) {
        let mnemonic = Mnemonic::from_entropy(&entropy, Language::English)
            .expect("Unable to generate mnemonic from entropy.")
            .into_phrase();
        self.setMnemonic(mnemonic);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::wallet::keystore::KeyStore;

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
    fn test_keystore_from_mnemonic() {
        let keyStore = KeyStore::fromMnemonic(MNEMONIC.to_string());
        assert_eq!(keyStore.mnemonic, MNEMONIC);
        assert_eq!(keyStore.seed, SEED);
        assert_eq!(keyStore.entropy, ENTROPY)
    }
}
