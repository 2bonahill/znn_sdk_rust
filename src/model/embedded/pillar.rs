use crate::model::primitives::address::Address;
use anyhow::Result;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct PillarInfoList {
    pub count: u64,
    pub list: Vec<PillarInfo>,
}

impl PillarInfoList {
    pub fn from_json(json: Map<String, Value>) -> Result<Self> {
        let pil: PillarInfoList = serde_json::from_value(serde_json::Value::Object(json))?;
        Ok(pil)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct PillarInfo {
    pub name: String,
    pub rank: u64,
    pub r#type: u32,
    #[serde(deserialize_with = "deserialize_address")]
    pub ownerAddress: Address,
    #[serde(deserialize_with = "deserialize_address")]
    pub producerAddress: Address,
    #[serde(deserialize_with = "deserialize_address")]
    pub withdrawAddress: Address,
    pub giveMomentumRewardPercentage: u64,
    pub giveDelegateRewardPercentage: u64,
    pub isRevocable: bool,
    pub revokeCooldown: u64,
    pub revokeTimestamp: u64,
    pub currentStats: PillarEpochStats,
    pub weight: u64,
}

#[allow(non_snake_case, non_upper_case_globals)]
impl PillarInfo {
    pub const unknownType: u64 = 0;
    pub const legacyPillarType: u64 = 1;
    pub const regularPillarType: u64 = 2;

    pub fn from_json(json: Map<String, Value>) -> Result<Self> {
        let pi: PillarInfo = serde_json::from_value(serde_json::Value::Object(json))?;
        Ok(pi)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct PillarEpochStats {
    pub producedMomentums: u32,
    pub expectedMomentums: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct DelegationInfo {
    pub name: String,
    pub status: u32,
    pub weight: u64,
}

impl DelegationInfo {
    pub fn from_json(json: Map<String, Value>) -> Result<Self> {
        let di: DelegationInfo = serde_json::from_value(serde_json::Value::Object(json))?;
        Ok(di)
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
