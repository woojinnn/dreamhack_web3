use alloy::{
    network::ReceiptResponse,
    primitives::{utils::parse_ether, Address},
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::{TransactionInput, TransactionRequest},
    signers::local::PrivateKeySigner,
    sol,
};
use eyre::Result;
use dreamhack_utils::{
    challenge::info::{
        fetch_prob_info, get_info_value, read_info_url_from_arguments, rpc_url_from_info_url,
    },
    forge::build_contract_bytecode,
};
use std::path::Path;

sol!("./Solution.sol");

sol! {
    #[sol(rpc)]
    interface LevelContract {
        function register(address voter) external payable;
        function getResult() external view returns (string memory);
    }
}

const LEVEL_ADDRESS_KEY: &str = "level_contract_address";
const PRIVATE_KEY: &str = "user_private_key";

#[tokio::main]
async fn main() -> Result<()> {
    let info_url = read_info_url_from_arguments()?;
    let info_map = fetch_prob_info(&info_url).await?;
    let rpc_url = rpc_url_from_info_url(&info_url)?;

    let user_private_key = get_info_value(&info_map, PRIVATE_KEY)?;
    let level_contract_address = get_info_value(&info_map, LEVEL_ADDRESS_KEY)?;

    let signer: PrivateKeySigner = user_private_key.parse()?;
    let level_address: Address = level_contract_address.parse()?;

    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect_http(rpc_url.parse()?);

    let deploy_input = build_contract_bytecode(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        "Solution.sol",
        "Solve",
        Solve::constructorCall {
            addr: level_address,
        },
    )?;
    let deploy_tx = TransactionRequest::default()
        .create()
        .input(TransactionInput::new(deploy_input.into()));

    let deploy_receipt = provider
        .send_transaction(deploy_tx)
        .await?
        .get_receipt()
        .await?;
    let solve_address = deploy_receipt.contract_address().unwrap();
    println!("Deployed solution contract at: {solve_address}");

    let level = LevelContract::new(level_address, &provider);
    let _call_register = level
        .register(solve_address)
        .value(parse_ether("1")?)
        .send()
        .await?
        .get_receipt()
        .await?;

    Ok(())
}
