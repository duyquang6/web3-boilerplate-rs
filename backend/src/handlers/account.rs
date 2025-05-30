use std::time::SystemTime;

use alloy::{providers::Provider, rpc::types::Header};
use anyhow::anyhow;
use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;

use crate::{
    db::EthAccountBalance,
    error::{NotFoundError, Result, ValidateError},
};
use crate::{eth::ZERO_ADDRESS, state::AppState};

#[derive(Serialize)]
pub struct AccountResponse {
    address: String,
    current_block: u64,
    gas_price: u128,
    balance: String,
}

const BLOCK_HEADER_CACHE_KEY: &str = "block";
const BLOCK_MINE_DURATION: u64 = 12;

const GAS_PRICE_CACHE_KEY: &str = "block";
const GAS_PRICE_TTL: u64 = 60 * 5; // 5 minutes

pub async fn get_account_info(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<AccountResponse>> {
    // Validate the Ethereum address format
    if !address.starts_with("0x") || address.len() != 42 {
        return Err(ValidateError("Invalid Ethereum address format".to_string()).into());
    }

    tracing::info!("Fetching latest block header from provider.");
    let current_block = state
        .eth_provider
        .get_block_by_number(alloy::eips::BlockNumberOrTag::Latest)
        .await?
        .ok_or_else(|| NotFoundError("Latest block not found".to_string()))?;

    let gas_price = state.eth_provider.get_gas_price().await?;
    let eth_address = address.parse()?;
    let balance = state
        .eth_provider
        .get_balance(eth_address)
        .await?
        .to_string();

    let erc20_balance_decimal = balance.parse()?;
    EthAccountBalance::upsert(
        &state.pg_pool,
        &address.to_string().to_lowercase(),
        ZERO_ADDRESS,
        erc20_balance_decimal,
    )
    .await?;

    Ok(Json(AccountResponse {
        address,
        current_block: current_block.header.number,
        gas_price,
        balance,
    }))
}
