use anyhow::Result;
use bech32::{self, FromBase32, ToBase32, Variant};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Address {
    hrp: String,
    core: Vec<u8>,
    // pub address: String, // TODO: REMOVE, does not belong to the SDK
}

impl Address {
    const prefix: &'static str = "z";
    const addressLength: u8 = 40;
    const userByte: u8 = 0;
    const contractByte: u8 = 1;
    const coreSize: u8 = 20;

    pub fn new(hrp: String, core: Vec<u8>) -> Self {
        Self { hrp, core }
    }

    pub fn from_string(address: String) -> Self {
        Self {
            core: vec![],
            hrp: Address::prefix.to_string(),
        }
    }

    pub fn to_string(&self) -> Result<String> {
        let bech32 = bech32::encode(&self.hrp, &self.core.to_base32(), Variant::Bech32)?;
        let bech32_utf16 = Vec::from_iter(bech32.encode_utf16());
        let address_string = String::from_utf16(&bech32_utf16)?;
        Ok(address_string)
    }

    pub fn parse(address: String) -> Result<Self> {
        let (hrp, data, _) = bech32::decode(&address).unwrap();
        let core = Vec::<u8>::from_base32(&data).unwrap();
        Ok(Address::new(hrp, core))
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
        let a: Address = Address::new(hrp, core);
        assert_eq!(a.to_string()?, address_string);
        Ok(())
    }

    #[test]
    fn test_address_parse() -> Result<()> {
        let hrp: String = "z".to_string();
        let core = [
            0, 37, 55, 74, 65, 159, 50, 115, 111, 97, 236, 197, 172, 64, 89, 210, 241, 181, 136, 77,
        ]
        .to_vec();
        let a: Address = Address::parse("z1qqjnwjjpnue8xmmpanz6csze6tcmtzzdtfsww7".to_string())?;
        assert_eq!(a.core, core);
        assert_eq!(a.hrp, hrp);
        Ok(())
    }
}
