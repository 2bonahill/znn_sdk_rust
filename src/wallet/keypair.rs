use crate::model::primitives::address::Address;

pub struct KeyPair {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
    address: Option<Address>,
}

impl KeyPair {
    pub fn new(secret_key: Vec<u8>, public_key: Vec<u8>, address: Option<Address>) -> Self {
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
}
