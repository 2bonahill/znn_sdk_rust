use anyhow::Result;
use bech32::{self, FromBase32, ToBase32, Variant};
use serde::{Deserialize, Serialize};

use crate::crypto;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Address {
    pub hrp: String,
    core: Vec<u8>,
}

#[allow(dead_code)]
impl Address {
    const PREFIX: &'static str = "z";
    const ADDRESS_LENGTH: u8 = 40;
    const USER_BYTE: u8 = 0;
    const CONTRACT_BYTE: u8 = 1;
    const CORE_SIZE: u8 = 20;

    pub fn new(hrp: String, core: Vec<u8>) -> Self {
        Self { hrp, core }
    }

    pub fn from_string(_address: String) -> Self {
        Self {
            core: vec![],
            hrp: Address::PREFIX.to_string(),
        }
    }

    pub fn from_public_key(public_key: [u8; 32]) -> Result<Self> {
        let core = crypto::crypto::derive_address_bytes_from_public_key(&public_key);
        Ok(Self {
            hrp: 'z'.to_string(),
            core: core,
        })
    }

    pub fn parse(address: String) -> Result<Self> {
        let (hrp, data, _) = bech32::decode(&address).unwrap();
        let core = Vec::<u8>::from_base32(&data).unwrap();
        Ok(Address::new(hrp, core))
    }

    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.core
    }

    pub fn to_string(&self) -> Result<String> {
        let bech32 = bech32::encode(&self.hrp, &self.core.to_base32(), Variant::Bech32)?;
        let bech32_utf16 = Vec::from_iter(bech32.encode_utf16());
        let address_string = String::from_utf16(&bech32_utf16)?;
        Ok(address_string)
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

    #[test]
    fn test_from_public_key() -> Result<()> {
        let hrp: String = "z".to_string();
        let core = [
            0, 37, 55, 74, 65, 159, 50, 115, 111, 97, 236, 197, 172, 64, 89, 210, 241, 181, 136, 77,
        ]
        .to_vec();
        let public_key: [u8; 32] = [
            62, 19, 215, 35, 141, 14, 118, 138, 86, 125, 206, 132, 181, 73, 21, 242, 50, 63, 45,
            205, 14, 249, 167, 22, 217, 198, 26, 190, 214, 49, 186, 16,
        ];
        let a: Address = Address::from_public_key(public_key)?;
        assert_eq!(a.core, core);
        assert_eq!(a.hrp, hrp);
        Ok(())
    }
}
