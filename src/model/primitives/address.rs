use anyhow::Result;
use bech32::{self, FromBase32, ToBase32, Variant};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Address {
    hrp: String,
    core: Vec<u8>,
    pub address: String, // TODO: REMOVE, does not belong to the SDK
}

impl Address {
    const prefix: &'static str = "z";
    const addressLength: u8 = 40;
    const userByte: u8 = 0;
    const contractByte: u8 = 1;
    const coreSize: u8 = 20;

    pub fn new(hrp: String, core: Vec<u8>, address: String) -> Self {
        Self { hrp, core, address }
    }

    pub fn from_string(address: String) -> Self {
        Self {
            core: vec![],
            hrp: Address::prefix.to_string(),
            address: address,
        }
    }

    pub fn to_string(&self) -> Result<String> {
        let bech32 = bech32::encode(&self.hrp, &self.core.to_base32(), Variant::Bech32)?;
        let bech32_utf16 = Vec::from_iter(bech32.encode_utf16());
        let address_string = String::from_utf16(&bech32_utf16)?;
        Ok(address_string)
    }

    pub fn bech_playground() {
        let encoded = bech32::encode(
            "bech32",
            vec![0x00, 0x01, 0x02].to_base32(),
            Variant::Bech32,
        )
        .unwrap();
        assert_eq!(encoded, "bech321qqqsyrhqy2a".to_string());
        let (hrp, data, variant) = bech32::decode(&encoded).unwrap();
        assert_eq!(hrp, "bech32");
        assert_eq!(
            Vec::<u8>::from_base32(&data).unwrap(),
            vec![0x00, 0x01, 0x02]
        );
        assert_eq!(variant, Variant::Bech32);
    }

    /// Get a reference to the address's address.
    pub fn to_string_old(&self) -> &str {
        self.address.as_ref()
    }
}

#[cfg(test)]
mod tests {

    use anyhow::Result;

    use crate::model::primitives::address::Address;

    #[test]
    fn test_address() -> Result<()> {
        let hrp: String = "z".to_string();
        let core = [
            0, 37, 55, 74, 65, 159, 50, 115, 111, 97, 236, 197, 172, 64, 89, 210, 241, 181, 136, 77,
        ]
        .to_vec();
        let address_string = "z1qqjnwjjpnue8xmmpanz6csze6tcmtzzdtfsww7".to_string();
        let a: Address = Address::new(hrp, core, "lol".to_string());
        assert_eq!(a.to_string()?, address_string);
        Ok(())
    }
}
