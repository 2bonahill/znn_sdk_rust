use crate::client::websocket::WsClient;
use crate::error::Error;
use crate::model::embedded::common::RewardHistoryList;
use crate::model::embedded::common::UncollectedReward;
use crate::model::embedded::pillar::DelegationInfo;
use crate::model::embedded::pillar::PillarInfo;
use crate::model::embedded::pillar::PillarInfoList;
use crate::model::primitives::address::Address;
use serde_json::Map;
use serde_json::Value;

pub struct PillarApi {}

impl PillarApi {
    pub async fn get_qsr_registration_cost(client: &WsClient) -> Result<u64, Error> {
        let response = client
            .send_request("embedded.pillar.getQsrRegistrationCost", vec![])
            .await?
            .as_u64();

        match response {
            Some(r) => Ok(r),
            None => Err(Error::ApiError(format!(
                "embedded.pillar.getQsrRegistrationCost returned Null"
            ))),
        }
    }

    pub async fn check_name_availability(client: &WsClient, name: String) -> Result<bool, Error> {
        let response = client
            .send_request(
                "embedded.pillar.checkNameAvailability",
                vec![serde_json::to_value(name)?],
            )
            .await?
            .as_bool();

        match response {
            Some(r) => Ok(r),
            None => Err(Error::ApiError(format!(
                "embedded.pillar.checkNameAvailability returned Null"
            ))),
        }
    }

    pub async fn get_all(
        client: &WsClient,
        page_index: u8,
        page_size: u8,
    ) -> Result<PillarInfoList, Error> {
        let response = client
            .send_request(
                "embedded.pillar.getAll",
                vec![
                    serde_json::to_value(page_index)?,
                    serde_json::to_value(page_size)?,
                ],
            )
            .await?;

        match response.as_object() {
            Some(r) => {
                let pil: PillarInfoList = PillarInfoList::from_json(r.clone())?;
                Ok(pil)
            }
            None => Err(Error::ApiError(format!(
                "embedded.pillar.getAll returned Null for page index {} and size {}",
                page_index, page_size
            ))),
        }
    }

    pub async fn get_by_owner(
        client: &WsClient,
        address: Address,
    ) -> Result<Vec<PillarInfo>, Error> {
        let response = client
            .send_request(
                "embedded.pillar.getByOwner",
                vec![serde_json::to_value(address.to_string()?)?],
            )
            .await?;

        let mut result: Vec<PillarInfo> = Vec::new();

        match response.as_array() {
            Some(r) => {
                for i in r {
                    let pi_map: Map<String, Value> = i.as_object().unwrap().clone();
                    let pi: PillarInfo = PillarInfo::from_json(pi_map)?;
                    result.push(pi);
                }
                Ok(result)
            }
            None => Err(Error::ApiError(format!(
                "embedded.pillar.getByOwner returned Null for address {}",
                address.to_string()?
            ))),
        }
    }

    pub async fn get_by_name(client: &WsClient, name: &str) -> Result<Option<PillarInfo>, Error> {
        let response = client
            .send_request(
                "embedded.pillar.getByName",
                vec![serde_json::to_value(name)?],
            )
            .await?;

        if response.is_object() {
            let pi: PillarInfo = PillarInfo::from_json(response.as_object().unwrap().clone())?;
            return Ok(Some(pi));
        }
        Ok(None)
    }

    pub async fn get_delegated_pillar(
        client: &WsClient,
        address: Address,
    ) -> Result<DelegationInfo, Error> {
        let response: Map<String, Value> = client
            .send_request(
                "embedded.pillar.getDelegatedPillar",
                vec![serde_json::to_value(address.to_string()?)?],
            )
            .await?
            .as_object()
            .unwrap()
            .clone();
        let di: DelegationInfo = DelegationInfo::from_json(response)?;
        Ok(di)
    }

    pub async fn get_deposited_qsr(client: &WsClient, address: Address) -> Result<u64, Error> {
        let response: u64 = client
            .send_request(
                "embedded.pillar.getDepositedQsr",
                vec![serde_json::to_value(address.to_string()?)?],
            )
            .await?
            .as_u64()
            .unwrap();
        Ok(response)
    }

    pub async fn get_uncollected_reward(
        client: &WsClient,
        address: Address,
    ) -> Result<UncollectedReward, Error> {
        let response: Map<String, Value> = client
            .send_request(
                "embedded.pillar.getUncollectedReward",
                vec![serde_json::to_value(address.to_string()?)?],
            )
            .await?
            .as_object()
            .unwrap()
            .clone();
        let ur: UncollectedReward = UncollectedReward::from_json(response)?;
        Ok(ur)
    }

    pub async fn get_frontier_reward_by_page(
        client: &WsClient,
        address: Address,
        page_index: u8,
        page_size: u8,
    ) -> Result<RewardHistoryList, Error> {
        let response: Map<String, Value> = client
            .send_request(
                "embedded.pillar.getFrontierRewardByPage",
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
        let rhl: RewardHistoryList = RewardHistoryList::from_json(response)?;
        Ok(rhl)
    }
}
