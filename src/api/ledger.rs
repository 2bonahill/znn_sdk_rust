use crate::client::websocket::WsClient;
use crate::error::Error;
use crate::model::nom::account_info::AccountInfo;
use crate::model::primitives::address::Address;
use serde_json::Map;
use serde_json::Value;

pub struct LedgerApi {}

impl LedgerApi {
    pub async fn get_account_info_by_address(
        client: &WsClient,
        address: Address,
    ) -> Result<AccountInfo, Error> {
        let address_string = address.to_string()?;

        let response: Map<String, Value> = client
            .send_request(
                "ledger.getAccountInfoByAddress",
                vec![serde_json::to_value(&address_string)?],
            )
            .await?
            .as_object()
            .unwrap()
            .clone();

        // dbg!("ledger::getAccountInfoByAddress: {}", &response);
        let ai: AccountInfo = AccountInfo::from_json(response)?;
        Ok(ai)
    }
}
