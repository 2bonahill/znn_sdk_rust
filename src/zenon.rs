use std::sync::Arc;

use crate::{
    api::{embedded::EmbeddedApi, ledger::LedgerApi},
    client::websocket::WsClient,
    error::Error,
};

pub struct Zenon {
    pub client: Arc<WsClient>,
    pub embedded: EmbeddedApi,
    pub ledger: LedgerApi,
}

impl Zenon {
    pub async fn init(url: &str) -> Result<Self, Error> {
        let client = Arc::new(WsClient::initialize(url).await?);
        let embedded = EmbeddedApi::new(client.clone());
        let ledger: LedgerApi = LedgerApi::new(client.clone());

        let znn = Zenon {
            client,
            embedded,
            ledger,
        };

        Ok(znn)
    }
}
