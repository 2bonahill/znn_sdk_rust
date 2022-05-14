use anyhow::Result;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_ws_client::{types::ParamsSer, WsClientBuilder};
use serde_json::Value;

pub struct WsClient {
    client: jsonrpsee_ws_client::WsClient,
}

impl WsClient {
    pub async fn initialize(url: &str) -> Result<Self> {
        Ok(Self {
            client: WsClientBuilder::default().build(url).await?,
        })
    }

    pub fn is_connected(&self) -> bool {
        self.client.is_connected()
    }

    pub async fn send_request(
        &self,
        method: &str,
        params: Vec<Value>,
    ) -> Result<serde_json::Map<String, serde_json::Value>> {
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
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::websocket::WsClient;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_initialize() -> Result<()> {
        let client = WsClient::initialize("ws://nodes.zenon.place:35998").await?;
        assert_eq!(client.is_connected(), true);
        Ok(())
    }
}
