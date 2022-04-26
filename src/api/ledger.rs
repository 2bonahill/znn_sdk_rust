use crate::client::websocket::WsClient;
use crate::model::primitives::address::Address;
use anyhow::Result;
use serde_json::Map;
use serde_json::Value;
// use crate::model::nom::account_info::AccountInfo;
// use crate::model::primitives::address::Address;

pub struct LedgerApi {}

impl LedgerApi {
    pub async fn get_account_info_by_address(
        client: &WsClient,
        address: Address,
    ) -> Result<Map<String, Value>> {
        let response = client
            .sendRequest(
                "ledger.getAccountInfoByAddress",
                vec![serde_json::to_value(address.to_string())?],
            )
            .await?;

        Ok(response)
    }
}
