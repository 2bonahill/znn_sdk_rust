use serde::{Deserialize, Deserializer, Serialize};

use crate::model::primitives::{address::Address, token_standard::TokenStandard};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub domain: String,
    pub totalSupply: u64,
    pub decimals: u64,
    #[serde(deserialize_with = "deserialize_address")]
    pub owner: Address,
    #[serde(deserialize_with = "deserialize_token_standard")]
    pub tokenStandard: TokenStandard,
    pub maxSupply: u64,
    pub isBurnable: bool,
    pub isMintable: bool,
    pub isUtility: bool,
}

fn deserialize_address<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let buf: String = String::deserialize(deserializer)?;
    let address: Address = Address::new(buf.clone());
    Ok(address)
}

fn deserialize_token_standard<'de, D>(deserializer: D) -> Result<TokenStandard, D::Error>
where
    D: Deserializer<'de>,
{
    let buf: String = String::deserialize(deserializer)?;
    let ts: TokenStandard = TokenStandard::new(buf.clone());
    Ok(ts)
}
