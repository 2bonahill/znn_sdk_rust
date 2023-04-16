use crate::api_tests::test_data;
use anyhow::Result;
use pretty_assertions::assert_eq;
use znn_sdk_rust::{
    client::websocket::WsClient,
    model::{
        embedded::{
            common::RewardHistoryList,
            pillar::{DelegationInfo, PillarInfoList},
        },
        nom::account_info::AccountInfo,
        primitives::address::Address,
    },
};

#[tokio::test]
pub async fn test_ws_client() -> Result<()> {
    let client = WsClient::initialize("invalid URL!").await;
    assert_eq!(client.is_err(), true);
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

#[tokio::test]
pub async fn test_pillar_api_get_all() -> Result<()> {
    let client: WsClient = WsClient::initialize(test_data::TEST_NODE).await?;
    assert_eq!(client.is_connected(), true);
    let _pil: PillarInfoList = znn_sdk_rust::api::embedded::Pillar::get_all(&client, 1, 3).await?;
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_api_get_qsr_registration_cost() -> Result<()> {
    let client = WsClient::initialize(test_data::TEST_NODE).await?;
    let _result: u64 =
        znn_sdk_rust::api::embedded::Pillar::get_qsr_registration_cost(&client).await?;
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_check_name_availability() -> Result<()> {
    let client = WsClient::initialize(test_data::TEST_NODE).await?;

    // should be available
    let result: bool = znn_sdk_rust::api::embedded::Pillar::check_name_availability(
        &client,
        "SultanOfStaking".to_string(),
    )
    .await?;
    assert_eq!(result, false);

    // should not be available
    let result: bool = znn_sdk_rust::api::embedded::Pillar::check_name_availability(
        &client,
        "SultanOfStakingsXYZDoesNotExist123".to_string(),
    )
    .await?;
    assert_eq!(result, true);

    Ok(())
}

#[tokio::test]
pub async fn test_pillar_get_by_owner() -> Result<()> {
    let client: WsClient = WsClient::initialize(test_data::TEST_NODE).await?;
    assert_eq!(client.is_connected(), true);
    let a = Address::parse("z1qzlaadsmar8pm0rdfwkctvxc8n2g5gaadxvmqj")?;
    let _ai = znn_sdk_rust::api::embedded::Pillar::get_by_owner(&client, a).await?;
    // dbg!(&_ai);
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_get_by_name() -> Result<()> {
    let client: WsClient = WsClient::initialize(test_data::TEST_NODE).await?;
    assert_eq!(client.is_connected(), true);
    let name1 = "SultanOfStaking";
    let _pi1 = znn_sdk_rust::api::embedded::Pillar::get_by_name(&client, name1).await?;
    assert_eq!(_pi1.is_some(), true);

    let name2 = "NameThatDoesNotExist!";
    let _pi2 = znn_sdk_rust::api::embedded::Pillar::get_by_name(&client, name2).await?;
    assert_eq!(_pi2.is_none(), true);
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_get_delegated_pillar() -> Result<()> {
    let client: WsClient = WsClient::initialize(test_data::TEST_NODE).await?;
    assert_eq!(client.is_connected(), true);
    let a = Address::parse("z1qrgr0e2u8y4pg4lzjr3fr62g8q4letyuntcvt5")?;
    let _di: DelegationInfo =
        znn_sdk_rust::api::embedded::Pillar::get_delegated_pillar(&client, a).await?;
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_get_deposited_qsr() -> Result<()> {
    let client: WsClient = WsClient::initialize(test_data::TEST_NODE).await?;
    assert_eq!(client.is_connected(), true);
    let a = Address::parse("z1qrgr0e2u8y4pg4lzjr3fr62g8q4letyuntcvt5")?;
    let _ai = znn_sdk_rust::api::embedded::Pillar::get_deposited_qsr(&client, a).await?;
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_get_uncollected_reward() -> Result<()> {
    let client: WsClient = WsClient::initialize(test_data::TEST_NODE).await?;
    assert_eq!(client.is_connected(), true);
    let a = Address::parse("z1qrgr0e2u8y4pg4lzjr3fr62g8q4letyuntcvt5")?;
    let _ur = znn_sdk_rust::api::embedded::Pillar::get_uncollected_reward(&client, a).await?;
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_get_frontier_reward_by_page() -> Result<()> {
    let client: WsClient = WsClient::initialize(test_data::TEST_NODE).await?;
    assert_eq!(client.is_connected(), true);
    let a = Address::parse("z1qrgr0e2u8y4pg4lzjr3fr62g8q4letyuntcvt5")?;
    let _rhl: RewardHistoryList =
        znn_sdk_rust::api::embedded::Pillar::get_frontier_reward_by_page(&client, a, 0, 5).await?;
    Ok(())
}

#[tokio::test]
pub async fn test_plasma_get() -> Result<()> {
    let client: WsClient = WsClient::initialize(test_data::TEST_NODE).await?;
    assert_eq!(client.is_connected(), true);
    let a = Address::parse("z1qrgr0e2u8y4pg4lzjr3fr62g8q4letyuntcvt5")?;
    let _pi = znn_sdk_rust::api::embedded::Plasma::get(&client, a).await?;
    Ok(())
}

#[tokio::test]
pub async fn test_plasma_get_entries_by_address() -> Result<()> {
    let client: WsClient = WsClient::initialize(test_data::TEST_NODE).await?;
    assert_eq!(client.is_connected(), true);
    let a = Address::parse("z1qrgr0e2u8y4pg4lzjr3fr62g8q4letyuntcvt5")?;
    let _fel =
        znn_sdk_rust::api::embedded::Plasma::get_entries_by_address(&client, a, 1, 5).await?;
    // dbg!(&_fel);
    Ok(())
}

#[tokio::test]
pub async fn test_plasma_get_required_pow_for_account_block() -> Result<()> {
    let client: WsClient = WsClient::initialize(test_data::TEST_NODE).await?;
    assert_eq!(client.is_connected(), true);
    let a = Address::parse("z1qz5fskcw8q6zndyu2w5eps9cyk3ekn9ecvcngd")?;
    let a_to = Address::parse("z1qqdtl63rkhap72nlaymtkemlchwv0ns9ksfjyn")?;
    let _rpfa = znn_sdk_rust::api::embedded::Plasma::get_required_pow_for_account_block(
        &client,
        a,
        1,
        a_to,
        vec![],
    )
    .await?;
    Ok(())
}

// #[tokio::test]
// pub async fn test_sentinel_get_by_owner() -> Result<()> {
//     let client: WsClient = WsClient::initialize(test_data::TEST_NODE).await?;
//     assert_eq!(client.is_connected(), true);
//     let a = Address::parse("z1qz5fskcw8q6zndyu2w5eps9cyk3ekn9ecvcngd")?;
//     let _rpfa = znn_sdk_rust::api::embedded::Sentinel::get_by_owner(&client, a).await?;
//     Ok(())
// }
