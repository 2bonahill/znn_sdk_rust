use crate::client::websocket::WsClient;
use crate::error::Error;
use crate::model::embedded::pillar::PillarInfoList;
use serde_json::Map;
use serde_json::Value;

pub struct PillarApi {}

impl PillarApi {
    pub async fn get_all(
        client: &WsClient,
        page_index: u8,
        page_size: u8,
    ) -> Result<PillarInfoList, Error> {
        // let response: Map<String, Value> = client
        //     .send_request(
        //         "embedded.pillar.getAll",
        //         vec![
        //             serde_json::to_value(page_index)?,
        //             serde_json::to_value(page_size)?,
        //         ],
        //     )
        //     .await?;

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
        let response = client
            .send_request("embedded.pillar.getQsrRegistrationCost", vec![])
            .await?
            .as_u64()
            .unwrap();
        Ok(response)
    }
}
