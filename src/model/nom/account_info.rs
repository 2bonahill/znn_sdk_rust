use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;

use super::token::Token;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountInfo {
    pub address: String,
    #[serde(rename = "accountHeight")]
    pub blockCount: u32,
    #[serde(rename = "balanceInfoMap")]
    pub balanceInfoList: HashMap<String, BalanceInfoListItem>,
}

#[allow(non_snake_case)]
impl AccountInfo {
    pub fn from_json(json: Map<String, Value>) -> Result<Self> {
        let ai: AccountInfo = serde_json::from_value(serde_json::Value::Object(json))?;
        Ok(ai)
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceInfoListItem {
    pub balance: Option<u64>,
    pub token: Option<Token>,
    pub balanceWithDecimals: Option<f64>,
    pub balanceFormatted: Option<String>,
}

impl BalanceInfoListItem {}
