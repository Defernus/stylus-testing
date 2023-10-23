use std::sync::Arc;

use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::Provider,
    signers::{LocalWallet, Signer},
    types::U256,
};

use stylus_testing::{
    private_key::key_from_index,
    provider::{TestInnerProvider, TestProvider},
};

static CONTRACT_BYTES: &'static [u8] = include_bytes!("../../contracts/erc20.wasm");

abigen!(
    Erc20,
    r#"[
        function init(address gov, string calldata name, string calldata symbol, uint8 decimals) external
        function setMinter(address minter, bool is_active) external
        function setGov(address gov) external
        function mint(address account, uint256 amount) external
        function burn(uint256 amount) external
        function name() external view returns (string memory)
        function symbol() external view returns (string memory)
        function decimals() external view returns (uint8)
        function totalSupply() external view returns (uint256)
        function balanceOf(address account) external view returns (uint256)
        function transfer(address recipient, uint256 amount) external returns (bool)
        function allowance(address owner, address spender) external view returns (uint256)
        function approve(address spender, uint256 amount) external returns (bool)
        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool)
        function increaseAllowance(address spender, uint256 added_value) external returns (bool)
        function decreaseAllowance(address spender, uint256 subtracted_value) external returns (bool)
    ]"#
);

#[tokio::main]
async fn main() {
    let private_key = key_from_index(0);
    let wallet = LocalWallet::from_bytes(&private_key)
        .unwrap()
        .with_chain_id(4_u64);
    let test_provider = Provider::new(TestInnerProvider::new());

    let client = Arc::new(SignerMiddleware::new(test_provider, wallet.clone()));

    let token_addr = client.deploy_contract(CONTRACT_BYTES);

    let token = Erc20::new(token_addr, client.clone());

    println!("=== init ===");
    token
        .init(wallet.address(), "Bitcoin".into(), "BTC".into(), 8)
        .await
        .unwrap();

    println!("=== set_minter ===");
    token.set_minter(wallet.address(), true).await.unwrap();

    println!("=== mint ===");
    token.mint(wallet.address(), U256::from(3)).await.unwrap();

    println!("=== balance_of ===");
    let balance = token.balance_of(wallet.address()).await.unwrap();

    assert_eq!(balance, U256::from(3));

    println!("balance: {:?}", balance);
}
