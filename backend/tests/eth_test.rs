use alloy::{
    primitives::{Address, address},
    providers::Provider,
};
use backend::eth::*;

#[tokio::test]
async fn test_setup_provider() {
    let rpc_url = "https://1rpc.io/sepolia";
    let provider = setup_provider(rpc_url).await;
    assert!(provider.is_ok());
}

#[tokio::test]
async fn test_get_balance() {
    let rpc_url = "https://1rpc.io/sepolia";
    let provider = setup_provider(rpc_url).await.unwrap();
    let address: Address = address!("0xd27de11aaacd14c62fe689d214a67e9385e6f60c");

    let balance = provider.get_balance(address).await.unwrap();
    assert!(!balance.is_zero(), "Balance should be greater than zero");
}

#[tokio::test]
async fn test_get_erc20_balance() {
    let rpc_url = "https://1rpc.io/sepolia";
    let provider = setup_provider(rpc_url).await.unwrap();
    let address: Address = address!("0xd27de11aaacd14c62fe689d214a67e9385e6f60c");

    let token_address = address!("0xab809CB0aB6669d51f6189432f751f1a916a10cd");

    let contract = IERC20Instance::new(token_address, provider);

    let balance = contract.balanceOf(address).call().await.unwrap();
    assert!(!balance.is_zero(), "Balance should be zero");
}
