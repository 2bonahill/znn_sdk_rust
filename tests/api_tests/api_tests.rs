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
    zenon::Zenon,
};

#[tokio::test]
pub async fn test_new_api_design() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    let test = znn
        .embedded
        .pillar
        .get_qsr_registration_cost()
        .await
        .unwrap();
    dbg!(test);
    Ok(())
}

#[tokio::test]
pub async fn test_ws_client() -> Result<()> {
    let client = WsClient::initialize("invalid URL!").await;
    assert_eq!(client.is_err(), true);
    Ok(())
}

#[tokio::test]
pub async fn test_ledger_api() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    let a = Address::parse("z1qq0hffeyj0htmnr4gc6grd8zmqfvwzgrydt402")?;
    let _ai: AccountInfo = znn.ledger.get_account_info_by_address(a).await?;
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_api_get_all() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    let _pil: PillarInfoList = znn.embedded.pillar.get_all(1, 3).await?;
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_api_get_qsr_registration_cost() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    let _result: u64 = znn.embedded.pillar.get_qsr_registration_cost().await?;
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_check_name_availability() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    // should be available
    let result: bool = znn
        .embedded
        .pillar
        .check_name_availability("SultanOfStaking".to_string())
        .await?;
    assert_eq!(result, false);

    // should not be available
    let result: bool = znn
        .embedded
        .pillar
        .check_name_availability("SultanOfStakingsXYZDoesNotExist123".to_string())
        .await?;
    assert_eq!(result, true);

    Ok(())
}

#[tokio::test]
pub async fn test_pillar_get_by_owner() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    assert_eq!(znn.client.is_connected(), true);
    let a = Address::parse("z1qzlaadsmar8pm0rdfwkctvxc8n2g5gaadxvmqj")?;
    let _ai = znn.embedded.pillar.get_by_owner(a).await?;
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_get_by_name() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    assert_eq!(znn.client.is_connected(), true);
    let name1 = "SultanOfStaking";
    let _pi1 = znn.embedded.pillar.get_by_name(name1).await?;
    assert_eq!(_pi1.is_some(), true);

    let name2 = "NameThatDoesNotExist!";
    let _pi2 = znn.embedded.pillar.get_by_name(name2).await?;
    assert_eq!(_pi2.is_none(), true);
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_get_delegated_pillar() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    let a = Address::parse("z1qrgr0e2u8y4pg4lzjr3fr62g8q4letyuntcvt5")?;
    let _di: DelegationInfo = znn.embedded.pillar.get_delegated_pillar(a).await?;
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_get_deposited_qsr() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    let a = Address::parse("z1qrgr0e2u8y4pg4lzjr3fr62g8q4letyuntcvt5")?;
    let _ai = znn.embedded.pillar.get_deposited_qsr(a).await?;
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_get_uncollected_reward() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    let a = Address::parse("z1qrgr0e2u8y4pg4lzjr3fr62g8q4letyuntcvt5")?;
    let _ur = znn.embedded.pillar.get_uncollected_reward(a).await?;
    Ok(())
}

#[tokio::test]
pub async fn test_pillar_get_frontier_reward_by_page() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    let a = Address::parse("z1qrgr0e2u8y4pg4lzjr3fr62g8q4letyuntcvt5")?;
    let _rhl: RewardHistoryList = znn
        .embedded
        .pillar
        .get_frontier_reward_by_page(a, 0, 5)
        .await?;
    Ok(())
}

#[tokio::test]
pub async fn test_plasma_get() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    let a = Address::parse("z1qrgr0e2u8y4pg4lzjr3fr62g8q4letyuntcvt5")?;
    let _pi = znn.embedded.plasma.get(a).await?;
    Ok(())
}

#[tokio::test]
pub async fn test_plasma_get_entries_by_address() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    let a = Address::parse("z1qrgr0e2u8y4pg4lzjr3fr62g8q4letyuntcvt5")?;
    let _fel = znn.embedded.plasma.get_entries_by_address(a, 1, 5).await?;
    // dbg!(&_fel);
    Ok(())
}

#[tokio::test]
pub async fn test_plasma_get_required_pow_for_account_block() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    let a = Address::parse("z1qz5fskcw8q6zndyu2w5eps9cyk3ekn9ecvcngd")?;
    let a_to = Address::parse("z1qqdtl63rkhap72nlaymtkemlchwv0ns9ksfjyn")?;
    let _rpfa = znn
        .embedded
        .plasma
        .get_required_pow_for_account_block(a, 1, a_to, vec![])
        .await?;
    Ok(())
}

#[tokio::test]
pub async fn test_accelerator_get_all() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    let _pl = znn.embedded.accelerator.get_all(1, 50).await?;
    Ok(())
}

#[tokio::test]
pub async fn test_accelerator_get_prject_by_id() -> Result<()> {
    let znn = Zenon::init(test_data::TEST_NODE).await?;
    let _p = znn
        .embedded
        .accelerator
        .get_project_by_id("181d796b08ee088bf91bc180d1bf70ff9c3a7b2e763e5481ac959d7a131567fa")
        .await?;
    // dbg!(&_p);
    Ok(())
}
