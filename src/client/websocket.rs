use jsonrpsee_core::client::ClientT;
use jsonrpsee_ws_client::{types::ParamsSer, WsClientBuilder};
use serde_json::Map;
use serde_json::Value;

pub struct WsClient {
    client: jsonrpsee_ws_client::WsClient,
}

impl WsClient {
    pub async fn initialize(url: &str) -> Self {
        Self {
            client: WsClientBuilder::default().build(url).await.unwrap(),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.client.is_connected()
    }

    pub async fn sendRequest(
        &self,
        method: &str,
        params: Vec<Value>,
    ) -> serde_json::Map<String, serde_json::Value> {
        // let parameters = {
        //     let mut parameters = Vec::new();
        //     parameters.push(serde_json::to_value(1).unwrap());
        //     parameters.push(serde_json::to_value(10).unwrap());
        //     Some(ParamsSer::Array(parameters))
        // };

        let parameters = {
            let mut parameters = Vec::new();

            for p in params {
                parameters.push(p);
            }
            Some(ParamsSer::Array(parameters))
        };

        let response = self
            .client
            .request::<serde_json::Map<String, _>>(method, parameters)
            .await
            .unwrap();

        response
    }
}

#[cfg(test)]
mod tests {
    use crate::client::websocket::WsClient;
    use futures::executor::block_on;

    // #[test]
    // fn test_initialize() {
    //     block_on(WsClient::initialize("ws://nodes.zenon.place:35998"));
    //     assert_eq!(1, 1);
    // }
}
