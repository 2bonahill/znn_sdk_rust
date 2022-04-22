use crate::model::primitives::address::Address;

pub struct KeyFile {
    base_address: Address,
    crypto: _Crypto,
    timestamp: u64,
    version: u64,
}

impl KeyFile {}

struct _Crypto {}
