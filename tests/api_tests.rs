use pretty_assertions::assert_eq;
use znn_sdk_rust::client::websocket::WsClient;

// importing common module.
mod common;

#[tokio::test]
async fn test_api() {
    common::setup();
    let client: WsClient = WsClient::initialize("ws://nodes.zenon.place:35998").await;
    assert_eq!(client.is_connected(), true);

    let ps = znn_sdk_rust::api::embedded::Pillar::get_all(&client, 1, 10).await;
    dbg!("Number of pillars: {}", &ps["count"]);
    dbg!("result {}", &ps);

    let ai = znn_sdk_rust::api::ledger::Ledger::get_account_info_by_address(
        &client,
        "z1qq0hffeyj0htmnr4gc6grd8zmqfvwzgrydt402",
    );

    assert_eq!(1, 1);
    // TODO
}
