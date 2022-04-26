use anyhow::Result;
use pretty_assertions::assert_eq;
use znn_sdk_rust::{
    client::websocket::WsClient,
    model::{
        embedded::pillar::PillarInfoList, nom::account_info::AccountInfo,
        primitives::address::Address,
    },
};

mod common;

#[tokio::test]
async fn test_pillar_api() -> Result<()> {
    common::setup();
    let client: WsClient = WsClient::initialize("ws://nodes.zenon.place:35998").await?;
    assert_eq!(client.is_connected(), true);

    let pil: PillarInfoList = znn_sdk_rust::api::embedded::Pillar::get_all(&client, 1, 3).await?;

    dbg!("Pil: {}", &pil);
    assert_eq!(1, 1); // TODO: something better? :-)
    Ok(())
}

async fn test_ledger_api() -> Result<()> {
    common::setup();
    let client: WsClient = WsClient::initialize("ws://nodes.zenon.place:35998").await?;
    assert_eq!(client.is_connected(), true);
    let a = Address {
        address: "z1qq0hffeyj0htmnr4gc6grd8zmqfvwzgrydt402".to_string(),
    };
    let ai: AccountInfo =
        znn_sdk_rust::api::Ledger::get_account_info_by_address(&client, a).await?;

    dbg!("account info: {}", ai);
    assert_eq!(1, 1); // TODO: something better? :-)
    Ok(())
}
