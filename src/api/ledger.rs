use async_std::stream::Map;
use serde_json::Value;

use crate::client::websocket::WsClient;
// use crate::model::nom::account_info::AccountInfo;
use crate::model::primitives::address::Address;

pub struct Ledger {}

impl Ledger {
    pub async fn get_account_info_by_address(
        client: &WsClient,
        address: &str,
    ) -> Map<String, Value> {
        let response = client
            .sendRequest(
                "ledger.getAccountInfoByAddress",
                vec![serde_json::to_value(address.to_string()).unwrap()],
            )
            .await;
        // something along the line:
        // return AccountInfo::fromJson(response...);
        todo!();
    }
}
