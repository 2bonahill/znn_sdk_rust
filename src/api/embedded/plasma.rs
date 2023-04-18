use crate::client::websocket::WsClient;
use crate::error::Error;
use crate::model::embedded::plasma::FusionEntryList;
use crate::model::embedded::plasma::GetRequiredParam;
use crate::model::embedded::plasma::GetRequiredResponse;
use crate::model::embedded::plasma::PlasmaInfo;
use crate::model::primitives::address::Address;
use std::rc::Rc;
use std::vec;

pub struct PlasmaApi {
    pub client: Rc<WsClient>,
}

impl PlasmaApi {
    pub fn new(client: Rc<WsClient>) -> Self {
        Self { client }
    }

    pub async fn get(&self, address: Address) -> Result<PlasmaInfo, Error> {
        let response = self
            .client
            .as_ref()
            .send_request(
                "embedded.plasma.get",
                vec![serde_json::to_value(address.to_string()?)?],
            )
            .await?;

        match response.as_object() {
            Some(x) => {
                let pi: PlasmaInfo = PlasmaInfo::from_json(x.clone())?;
                Ok(pi)
            }
            None => Err(Error::ApiError(format!(
                "Nothing found for embedded.plasma.get using address: {}",
                address.to_string()?
            ))),
        }
    }

    pub async fn get_entries_by_address(
        &self,
        address: Address,
        page_index: u32,
        page_size: u32,
    ) -> Result<FusionEntryList, Error> {
        let response = self
            .client
            .as_ref()
            .send_request(
                "embedded.plasma.getEntriesByAddress",
                vec![
                    serde_json::to_value(address.to_string()?)?,
                    serde_json::to_value(page_index)?,
                    serde_json::to_value(page_size)?,
                ],
            )
            .await?
            .as_object()
            .unwrap()
            .clone();
        let fel: FusionEntryList = FusionEntryList::from_json(response)?;
        Ok(fel)
    }

    pub async fn get_required_pow_for_account_block(
        &self,
        address: Address,
        block_type: u32,
        to_address: Address,
        data: Vec<u64>,
    ) -> Result<GetRequiredResponse, Error> {
        let grp: GetRequiredParam = GetRequiredParam {
            address,
            blockType: block_type,
            toAddress: to_address,
            data,
        };
        let serialized_grp = serde_json::to_value(&grp).unwrap();

        let response = self
            .client
            .as_ref()
            .send_request(
                "embedded.plasma.getRequiredPoWForAccountBlock",
                vec![serialized_grp],
            )
            .await?
            .as_object()
            .unwrap()
            .clone();

        let grr: GetRequiredResponse = GetRequiredResponse::from_json(response)?;
        Ok(grr)
    }
}
