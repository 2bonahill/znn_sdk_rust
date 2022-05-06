extern crate znn_sdk_rust as znn;

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
    // api_example().await;
    // keyfile_example().await;
}

async fn api_example() {
    let client: WsClient = WsClient::initialize("ws://nodes.zenon.place:35998")
        .await
        .unwrap();
    println!("is the client connected?: {}", client.is_connected());
    assert_eq!(client.is_connected(), true);

    let pil: PillarInfoList = znn::api::embedded::Pillar::get_all(&client, 1, 10)
        .await
        .unwrap();
    println!("Number of pillars: {}", pil.count);

    let a = Address::parse("z1qq0hffeyj0htmnr4gc6grd8zmqfvwzgrydt402".to_string()).unwrap();
    let ai: AccountInfo = znn::api::Ledger::get_account_info_by_address(&client, a)
        .await
        .unwrap();
    dbg!("Account info: {}", ai);
}
