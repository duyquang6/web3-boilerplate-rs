use super::error::Result;
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::sol_types::sol;

pub const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

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
