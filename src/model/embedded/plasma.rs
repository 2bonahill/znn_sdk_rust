use crate::model::primitives::address::Address;
use anyhow::Result;
use serde::Deserializer;
use serde::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct PlasmaInfo {
    pub currentPlasma: u64,
    pub maxPlasma: u64,
    pub qsrAmount: u64,
}

impl PlasmaInfo {
    pub fn from_json(json: Map<String, Value>) -> Result<Self> {
        let pi: PlasmaInfo = serde_json::from_value(serde_json::Value::Object(json))?;
        Ok(pi)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct FusionEntryList {
    pub qsrAmount: u64,
    pub count: u64,
    pub list: Vec<FusionEntry>,
}

impl FusionEntryList {
    pub fn from_json(json: Map<String, Value>) -> Result<Self> {
        let fel: FusionEntryList = serde_json::from_value(serde_json::Value::Object(json))?;
        Ok(fel)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct FusionEntry {
    pub qsrAmount: u64,
    #[serde(deserialize_with = "deserialize_address")]
    pub beneficiary: Address,
    pub expirationHeight: u64,
    pub id: String, //TODO: make proper hash type
                    // pub isRevocable: bool,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct GetRequiredParam {
    #[serde(serialize_with = "address_serializer")]
    pub address: Address,
    pub blockType: u32,
    #[serde(serialize_with = "address_serializer")]
    pub toAddress: Address,
    pub data: Vec<u64>,
}

fn deserialize_address<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let buf: String = String::deserialize(deserializer)?;
    let address: Address = Address::parse(&buf).unwrap();
    Ok(address)
}

fn address_serializer<S>(a: &Address, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&a.to_string().unwrap())
}
