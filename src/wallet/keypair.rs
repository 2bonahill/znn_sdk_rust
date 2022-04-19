pub struct KeyPair {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub address: Vec<u8>,
}

impl KeyPair {
    pub fn new(secret_key: Vec<u8>, public_key: Vec<u8>, address: Vec<u8>) -> Self {
        Self {
            secret_key,
            public_key,
            address,
        }
    }

    pub fn get_secret_key(&self) -> &Vec<u8> {
        &self.secret_key
    }

    pub fn get_public_key(&self) -> &Vec<u8> {
        &self.public_key
    }

    pub fn get_address(&self) -> &Vec<u8> {
        &self.address
    }
}
