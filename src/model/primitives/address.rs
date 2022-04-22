pub struct Address {
    address: String,
}

impl Address {
    const PREFIX: &'static str = "Z";

    pub fn to_string(&self) -> &str {
        self.address.as_ref()
    }
}
