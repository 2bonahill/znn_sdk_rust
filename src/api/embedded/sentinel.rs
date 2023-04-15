use crate::error::Error;
use crate::{client::websocket::WsClient, model::primitives::address::Address};

/*
NOTE: Sentinels are not yet implemented by the NoM. This is why
their SDK implementation will be completed once they are implemented
*/

pub struct SentinelApi;

impl SentinelApi {
    pub async fn get_by_owner(client: &WsClient, address: Address) -> Result<(), Error> {
        let response = client
            .send_request(
                "embedded.sentinel.getByOwner",
                vec![serde_json::to_value(address.to_string()?)?],
            )
            .await?;

        match response.as_object() {
            Some(_) => {
                //let pi: PlasmaInfo = PlasmaInfo::from_json(r.clone())?;
                //Ok(pi)
                Ok(())
            }
            None => Err(Error::ApiError(format!(
                "embedded.sentinel.getByOwner returned Null for address: {}",
                address.to_string()?
            ))),
        }
    }
}
