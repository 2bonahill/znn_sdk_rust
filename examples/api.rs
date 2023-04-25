extern crate znn_sdk_rust as znn;

use std::thread;

use znn::{
    model::{
        embedded::pillar::PillarInfoList, nom::account_info::AccountInfo,
        primitives::address::Address,
    },
    zenon::Zenon,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // api_example().await;
    api_example_async().await;
}

async fn api_example() {
    // check https://deeznnodez.com for good nodes
    let znn = Zenon::init("ws://public.deeZNNodez.com:35998")
        .await
        .unwrap();
    println!("is the client connected?: {}", znn.client.is_connected());
    assert_eq!(znn.client.is_connected(), true);

    let pil: PillarInfoList = znn.embedded.pillar.get_all(1, 10).await.unwrap();
    println!("Number of pillars: {}", pil.count);

    let a = Address::parse("z1qq0hffeyj0htmnr4gc6grd8zmqfvwzgrydt402").unwrap();
    let ai: AccountInfo = znn.ledger.get_account_info_by_address(a).await.unwrap();
    dbg!("Account info: {}", ai);
}

async fn api_example_async() {
    let mut threads = vec![];
    for _ in 0..10 {
        threads.push(tokio::spawn(async {
            let znn = Zenon::init("ws://public.deeZNNodez.com:35998")
                .await
                .unwrap();

            let a = Address::parse("z1qq0hffeyj0htmnr4gc6grd8zmqfvwzgrydt402").unwrap();
            let ai: AccountInfo = znn.ledger.get_account_info_by_address(a).await.unwrap();
        }));
    }

    for t in threads {
        // Wait for the thread to finish. Returns a result.
        let _ = t.await.unwrap();
    }
}
