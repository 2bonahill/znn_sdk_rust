// use serde_json::Map;
// use serde_json::Value;

use serde::{Deserialize, Serialize};

use super::token::Token;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AccountInfo {
    pub address: String,
    pub blockCount: u32,
    pub balanceInfoList: Vec<BalanceInfoListItem>,
}

#[allow(non_snake_case)]
impl AccountInfo {}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceInfoListItem {
    pub token: Token,
    pub balance: u64,
    pub balanceWithDecimals: f64,
    pub balanceFormatted: String,
}

impl BalanceInfoListItem {}
