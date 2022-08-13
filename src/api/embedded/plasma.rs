use crate::client::websocket::WsClient;
use crate::error::Error;
use crate::model::embedded::common::RewardHistoryList;
use crate::model::embedded::common::UncollectedReward;
use crate::model::embedded::pillar::DelegationInfo;
use crate::model::embedded::pillar::PillarInfo;
use crate::model::embedded::pillar::PillarInfoList;
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
}

// #[derive(Serialize, Deserialize, Debug)]
// #[allow(non_snake_case)]
// pub struct UncollectedReward {
//     #[serde(deserialize_with = "deserialize_address")]
//     pub address: Address,
//     pub znnAmount: u64,
//     pub qsrAmount: u64,
// }
