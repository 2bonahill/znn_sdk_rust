use crate::client::websocket::WsClient;
use crate::model::embedded::pillar::PillarInfoList;
use anyhow::Result;
use serde_json::Map;
use serde_json::Value;

pub struct PillarApi {}

impl PillarApi {
    pub async fn get_all(
        client: &WsClient,
        page_index: u8,
        page_size: u8,
    ) -> Result<PillarInfoList> {
        let response: Map<String, Value> = client
            .send_request(
                "embedded.pillar.getAll",
                vec![
                    serde_json::to_value(page_index)?,
                    serde_json::to_value(page_size)?,
                ],
            )
            .await?;

        let pil: PillarInfoList = PillarInfoList::from_json(response)?;
        Ok(pil)
    }
}
