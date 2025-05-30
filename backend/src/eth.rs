use super::error::Result;
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::sol_types::sol;

/// The zero address in Ethereum, used to represent an native token.
pub const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

/// Sets up an Ethereum provider using the given RPC URL.
pub async fn setup_provider(rpc_url: &str) -> Result<DynProvider> {
    let rpc_url = rpc_url.parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    let chain_id = provider.get_chain_id().await?;
    tracing::info!("Success connect to chain_id {} network", chain_id);

    Ok(provider.erased())
}

// Import the generated contract bindings for IERC20
sol!(
    #[sol(rpc)]
    IERC20,
    "abi/IERC20.json"
);

pub use IERC20::IERC20Instance;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use alloy::{
        network::TxSigner,
        primitives::{Address, address},
        signers::local::{LocalSigner, PrivateKeySigner},
    };

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
}
