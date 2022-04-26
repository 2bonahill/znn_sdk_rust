use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    address: String,
}

impl Address {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    const PREFIX: &'static str = "Z";

    pub fn to_string(&self) -> &str {
        self.address.as_ref()
    }
}
