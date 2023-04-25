pub mod accelerator;
pub mod pillar;
pub mod plasma;
pub mod sentinel;

use std::sync::Arc;

use crate::client::websocket::WsClient;

use self::{
    accelerator::AcceleratorApi, pillar::PillarApi, plasma::PlasmaApi, sentinel::SentinelApi,
};

pub struct EmbeddedApi {
    pub client: Arc<WsClient>,
    pub accelerator: AcceleratorApi,
    pub pillar: PillarApi,
    pub plasma: PlasmaApi,
    pub sentinel: SentinelApi,
}

impl EmbeddedApi {
    pub fn new(client: Arc<WsClient>) -> Self {
        Self {
            client: client.clone(),
            accelerator: AcceleratorApi::new(client.clone()),
            pillar: PillarApi::new(client.clone()),
            plasma: PlasmaApi::new(client.clone()),
            sentinel: SentinelApi::new(client.clone()),
        }
    }
}
