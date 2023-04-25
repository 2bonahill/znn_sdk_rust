use crate::client::websocket::WsClient;
use crate::error::Error;
use crate::model::nom::account_info::AccountInfo;
use crate::model::primitives::address::Address;
use std::sync::Arc;

pub struct LedgerApi {
    pub client: Arc<WsClient>,
}

impl LedgerApi {
    pub fn new(client: Arc<WsClient>) -> Self {
        Self { client }
    }

    pub async fn get_account_info_by_address(
        &self,
        address: Address,
    ) -> Result<AccountInfo, Error> {
        let address_string = address.to_string()?;

        let response = self
            .client
            .as_ref()
            .send_request(
                "ledger.getAccountInfoByAddress",
                vec![serde_json::to_value(&address_string)?],
            )
            .await?;

        match response.as_object() {
            Some(r) => {
                let ai: AccountInfo = AccountInfo::from_json(r.clone())?;
                Ok(ai)
            }
            None => Err(Error::ApiError(format!(
                "ledger.getAccountInfoByAddress returned Null for address: {}",
                address.to_string()?
            ))),
        }
    }
}
