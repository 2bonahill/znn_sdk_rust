use crate::{
    crypto::crypto::sign, crypto::crypto::verify, error::Error, model::primitives::address::Address,
};
use anyhow::Result;

pub struct KeyPair {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
    _address: Address,
}

impl KeyPair {
    pub fn new(secret_key: Vec<u8>, public_key: Vec<u8>, _address: Address) -> Self {
        Self {
            secret_key,
            public_key,
            _address,
        }
    }

    pub fn get_secret_key(&self) -> &Vec<u8> {
        &self.secret_key
    }

    pub fn get_public_key(&self) -> &Vec<u8> {
        &self.public_key
    }

    pub fn address(&self) -> &Address {
        // TODO: check if address is null -> if so, create it
        &self._address
    }

    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        sign(message, &self.secret_key, &self.public_key)
    }

    pub fn verify(&self, signature: Vec<u8>, message: Vec<u8>) -> Result<(), Error> {
        verify(signature, message, self.public_key.clone())
    }
}
