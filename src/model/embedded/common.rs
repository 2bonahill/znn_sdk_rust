use crate::model::primitives::address::Address;
use anyhow::Result;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct UncollectedReward {
    #[serde(deserialize_with = "deserialize_address")]
    pub address: Address,
    pub znnAmount: u64,
    pub qsrAmount: u64,
}

impl UncollectedReward {
    pub fn from_json(json: Map<String, Value>) -> Result<Self> {
        let ur: UncollectedReward = serde_json::from_value(serde_json::Value::Object(json))?;
        Ok(ur)
    }
}

fn deserialize_address<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let buf: String = String::deserialize(deserializer)?;
    let address: Address = Address::parse(&buf).unwrap();
    Ok(address)
}
