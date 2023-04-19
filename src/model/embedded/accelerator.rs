use anyhow::Result;
use jsonrpsee_core::Serialize;
use serde::Deserialize;
use serde::Deserializer;
use serde_json::Value;

use crate::model::primitives::address::Address;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Project {
    pub creationTimestamp: u64,
    pub description: String,
    pub id: String,
    pub lastUpdateTimestamp: u64,
    pub name: String,
    #[serde(deserialize_with = "deserialize_address")]
    pub owner: Address,
    pub phaseIds: Vec<String>,
    pub phases: Vec<Phase>,
    pub qsrFundsNeeded: u64,
    pub status: u64,
    pub url: String,
    pub votes: Votes,
}

impl Project {
    pub fn from_json(json: Value) -> Result<Self> {
        let p: Project = serde_json::from_value(json)?;
        Ok(p)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectList {
    pub count: u64,
    pub list: Vec<Project>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Votes {
    pub id: String,
    pub no: u64,
    pub total: u64,
    pub yes: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Phase {
    pub phase: PhaseBreakdown,
    pub votes: Votes,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct PhaseBreakdown {
    pub acceptedTimestamp: u64,
    pub creationTimestamp: u64,
    pub description: String,
    pub id: String,
    pub name: String,
    pub projectID: String,
    pub qsrFundsNeeded: u64,
    pub status: u64,
    pub url: String,
    pub znnFundsNeeded: u64,
}

impl ProjectList {
    pub fn from_json(json: Value) -> Result<Self> {
        let pl: ProjectList = serde_json::from_value(json)?;
        Ok(pl)
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
