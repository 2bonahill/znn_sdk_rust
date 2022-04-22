use crate::client::websocket::WsClient;
use serde_json::Map;
use serde_json::Value;

pub struct PillarApi {}

impl PillarApi {
    pub async fn get_all(client: &WsClient, page_index: u8, page_size: u8) -> Map<String, Value> {
        let response = client
            .sendRequest(
                "embedded.pillar.getAll",
                vec![
                    serde_json::to_value(page_index).unwrap(),
                    serde_json::to_value(page_size).unwrap(),
                ],
            )
            .await;

        response
    }
}
