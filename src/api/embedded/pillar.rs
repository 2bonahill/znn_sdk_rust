use crate::client::websocket::WsClient;
use crate::error::Error;
use crate::model::embedded::pillar::PillarInfo;
use crate::model::embedded::pillar::PillarInfoList;
use crate::model::primitives::address::Address;
use serde_json::Map;
use serde_json::Value;

pub struct PillarApi {}

impl PillarApi {
    pub async fn get_all(
        client: &WsClient,
        page_index: u8,
        page_size: u8,
    ) -> Result<PillarInfoList, Error> {
        let response: Map<String, Value> = client
            .send_request(
                "embedded.pillar.getAll",
                vec![
                    serde_json::to_value(page_index)?,
                    serde_json::to_value(page_size)?,
                ],
            )
            .await?
            .as_object()
            .unwrap()
            .clone();

        let pil: PillarInfoList = PillarInfoList::from_json(response)?;
        Ok(pil)
    }

    pub async fn get_qsr_registration_cost(client: &WsClient) -> Result<u64, Error> {
        let response: u64 = client
            .send_request("embedded.pillar.getQsrRegistrationCost", vec![])
            .await?
            .as_u64()
            .unwrap();
        Ok(response)
    }

    pub async fn check_name_availability(client: &WsClient, name: String) -> Result<bool, Error> {
        let response: bool = client
            .send_request(
                "embedded.pillar.checkNameAvailability",
                vec![serde_json::to_value(name)?],
            )
            .await?
            .as_bool()
            .unwrap();
        Ok(response)
    }

    pub async fn get_by_owner(
        client: &WsClient,
        address: Address,
    ) -> Result<Vec<PillarInfo>, Error> {
        let response: Vec<Value> = client
            .send_request(
                "embedded.pillar.getByOwner",
                vec![serde_json::to_value(address.to_string()?)?],
            )
            .await?
            .as_array()
            .unwrap()
            .clone();

        let mut result: Vec<PillarInfo> = Vec::new();

        for i in &response {
            let pi_map: Map<String, Value> = i.as_object().unwrap().clone();
            let pi: PillarInfo = PillarInfo::from_json(pi_map)?;
            result.push(pi);
        }
        Ok(result)
    }
}
