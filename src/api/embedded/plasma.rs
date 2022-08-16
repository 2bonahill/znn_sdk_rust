use crate::client::websocket::WsClient;
use crate::error::Error;
use crate::model::embedded::common::RewardHistoryList;
use crate::model::embedded::common::UncollectedReward;
use crate::model::embedded::pillar::DelegationInfo;
use crate::model::embedded::pillar::PillarInfo;
use crate::model::embedded::pillar::PillarInfoList;
use crate::model::embedded::plasma::FusionEntryList;
use crate::model::embedded::plasma::PlasmaInfo;
use crate::model::primitives::address;
use crate::model::primitives::address::Address;
use serde_json::Map;
use serde_json::Value;

pub struct PlasmaApi {}

impl PlasmaApi {
    pub async fn get(client: &WsClient, address: Address) -> Result<PlasmaInfo, Error> {
        let response = client
            .send_request(
                "embedded.plasma.get",
                vec![serde_json::to_value(address.to_string()?)?],
            )
            .await?
            .as_object()
            .unwrap()
            .clone();
        let pi: PlasmaInfo = PlasmaInfo::from_json(response)?;
        Ok(pi)
    }

    pub async fn get_entries_by_address(
        client: &WsClient,
        address: Address,
        page_index: u32,
        page_size: u32,
    ) -> Result<FusionEntryList, Error> {
        let response = client
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
        // dbg!(&fel);
        Ok(fel)
    }
}
