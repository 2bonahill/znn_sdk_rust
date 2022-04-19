use crate::model::primitives::address::Address;

pub struct KeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    address: Option<Address>,
}

impl KeyPair {
    pub fn new(private_key: Vec<u8>, public_key: Vec<u8>, address: Option<Address>) -> Self {
        Self {
            private_key,
            public_key,
            address,
        }
    }

    pub fn get_private_key(&self) -> &Vec<u8> {
        &self.private_key
    }

    pub fn get_public_key(&self) -> &Vec<u8> {
        &self.public_key
    }
}
