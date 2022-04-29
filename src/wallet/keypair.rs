pub struct KeyPair {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
    _address: Vec<u8>, // TODO: this needs to be an address
}

impl KeyPair {
    pub fn new(secret_key: Vec<u8>, public_key: Vec<u8>, _address: Vec<u8>) -> Self {
        Self {
            secret_key,
            public_key,
            _address,
        }
    }

    pub fn get_secret_key(&self) -> &Vec<u8> {
        self.secret_key.as_ref()
    }

    pub fn get_public_key(&self) -> &Vec<u8> {
        self.public_key.as_ref()
    }

    pub fn address(&self) -> &Vec<u8> {
        // TODO: check if address is null -> if so, create it
        self._address.as_ref()
    }
}
