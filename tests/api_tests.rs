use anyhow::Result;
use pretty_assertions::assert_eq;
use znn_sdk_rust::{client::websocket::WsClient, model::embedded::pillar::PillarInfoList};

mod common;

#[tokio::test]
async fn test_api() -> Result<()> {
    common::setup();
    let client: WsClient = WsClient::initialize("ws://nodes.zenon.place:35998").await?;
    assert_eq!(client.is_connected(), true);

    let pil: PillarInfoList = znn_sdk_rust::api::embedded::Pillar::get_all(&client, 1, 3)
        .await
        .unwrap();

    dbg!("Pil: {}", &pil);
    // assert_eq!(pil.count, 3);

    // let ai = znn_sdk_rust::api::ledger::Ledger::get_account_info_by_address(
    //     &client,
    //     "z1qq0hffeyj0htmnr4gc6grd8zmqfvwzgrydt402",
    // );

    assert_eq!(1, 1);
    Ok(())
}
