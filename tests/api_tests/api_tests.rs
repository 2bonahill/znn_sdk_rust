use crate::api_tests::test_data;
use anyhow::Result;
use pretty_assertions::assert_eq;
use znn_sdk_rust::{
    client::websocket::WsClient,
    model::{
        embedded::pillar::PillarInfoList, nom::account_info::AccountInfo,
        primitives::address::Address,
    },
};

#[tokio::test]
pub async fn test_ws_client() -> Result<()> {
    let client = WsClient::initialize("invalid URL!").await;
    assert_eq!(client.is_err(), true);
    // TODO: this is not really a good test
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_api() -> Result<()> {
    let client: WsClient = WsClient::initialize(test_data::TEST_NODE).await?;
    assert_eq!(client.is_connected(), true);

    let _pil: PillarInfoList = znn_sdk_rust::api::embedded::Pillar::get_all(&client, 1, 3).await?;

    Ok(())
}

#[tokio::test]
pub async fn test_ledger_api() -> Result<()> {
    let client: WsClient = WsClient::initialize(test_data::TEST_NODE).await?;
    assert_eq!(client.is_connected(), true);
    let a = Address::parse("z1qq0hffeyj0htmnr4gc6grd8zmqfvwzgrydt402")?;
    let _ai: AccountInfo =
        znn_sdk_rust::api::Ledger::get_account_info_by_address(&client, a).await?;
    Ok(())
}
