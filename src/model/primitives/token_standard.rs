use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct TokenStandard {
    // pub hrp: String,
    // pub core: Vec<u32>,
    pub tokenStandard: String, // TODO: not part of the SDK
}

#[allow(dead_code)]
impl TokenStandard {
    pub fn new(token_standard: String) -> Self {
        Self {
            tokenStandard: token_standard,
        }
    }

    const PREFIX: &'static str = "zts";
    const CORE_SIZE: u8 = 10;
}
