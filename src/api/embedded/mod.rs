pub mod embedded;
pub mod pillar;
pub mod plasma;
pub mod sentinel;

use std::rc::Rc;

use crate::client::websocket::WsClient;

use self::{pillar::PillarApi, plasma::PlasmaApi, sentinel::SentinelApi};

pub struct EmbeddedApi {
    pub client: Rc<WsClient>,
    pub pillar: PillarApi,
    pub plasma: PlasmaApi,
    pub sentinel: SentinelApi,
}

impl EmbeddedApi {
    pub fn new(client: Rc<WsClient>) -> Self {
        Self {
            client: client.clone(),
            pillar: PillarApi::new(client.clone()),
            plasma: PlasmaApi::new(client.clone()),
            sentinel: SentinelApi::new(client.clone()),
        }
    }
}
