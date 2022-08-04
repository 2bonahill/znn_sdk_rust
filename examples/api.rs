extern crate znn_sdk_rust as znn;
use jsonrpsee_types::ParamsSer;
use serde_json::json;
use znn::{
    client::websocket::WsClient,
    model::{
        embedded::pillar::PillarInfoList, nom::account_info::AccountInfo,
        primitives::address::Address,
    },
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    api_example().await;
    api_call_example().await;
}

async fn api_example() {
    // check https://deeznnodez.com for good nodes
    // ws://nodes.zenon.place:35998
    // ws://public.deeZNNodez.com:35998
    let client: WsClient = WsClient::initialize("ws://public.deeZNNodez.com:35998")
        .await
        .unwrap();
    println!("is the client connected?: {}", client.is_connected());
    assert_eq!(client.is_connected(), true);

    let pil: PillarInfoList = znn::api::embedded::Pillar::get_all(&client, 1, 10)
        .await
        .unwrap();
    println!("Number of pillars: {}", pil.count);

    let a = Address::parse("z1qq0hffeyj0htmnr4gc6grd8zmqfvwzgrydt402").unwrap();
    let ai: AccountInfo = znn::api::Ledger::get_account_info_by_address(&client, a)
        .await
        .unwrap();
    dbg!("Account info: {}", ai);
}

async fn api_call_example() {
    println!("hallo from generic_api_call");
    let client = WsClient::initialize("ws://public.deeZNNodez.com:35998")
        .await
        .unwrap();
    // serde_json::to_value(1).unwrap()
    let r = client
        .send_request(
            "embedded.token.getAll",
            vec![
                serde_json::to_value(1).unwrap(),
                serde_json::to_value(2).unwrap(),
            ],
        )
        .await
        .unwrap();
    dbg!("Response: {}", r);
}
