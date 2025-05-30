use std::time::SystemTime;
use std::convert::TryFrom;

use alloy::{consensus::BlockHeader, providers::Provider};
use anyhow::anyhow;
use axum::{
    Json,
    extract::{Path, State},
};
use redis::AsyncCommands;
use serde::Serialize;
use rust_decimal::Decimal;

use crate::error::{NotFoundError, Result, ValidateError};
use crate::eth::ZERO_ADDRESS;
use crate::state::AppState;

use super::utils;

/// Response structure for account information
#[derive(Serialize)]
pub struct AccountResponse {
    address: String,
    current_block: u64,
    gas_price: u128,
    balance: String,
}

/// Fetches the current block number from cache or provider
async fn get_current_block_number(state: &AppState) -> Result<u64> {
    let mut conn = state.cache.get_conn().await?;
    
    match conn.get(utils::CURRENT_BLOCK_NUMBER_CACHE_KEY).await {
        Ok(Some(block_number)) => {
            tracing::info!("Using cached block number: {}", block_number);
            Ok(block_number)
        }
        Ok(None) => {
            tracing::info!("Cached block number not found, fetching latest block header from provider.");
            fetch_and_cache_block_number(state).await
        }
        Err(e) => {
            tracing::error!("Failed to get cached block number: {}", e);
            fetch_and_cache_block_number(state).await
        }
    }
}

/// Fetches the latest block number from provider and caches it
async fn fetch_and_cache_block_number(state: &AppState) -> Result<u64> {
    let current_block = state
        .eth_provider
        .get_block_by_number(alloy::eips::BlockNumberOrTag::Latest)
        .await?
        .ok_or_else(|| NotFoundError("Latest block not found".to_string()))?;

    let block_number = current_block.header.number;
    let epoch_now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_| anyhow!("Failed to get current time"))?
        .as_secs();
    
    let cache_ttl = utils::BLOCK_MINE_DURATION as i64 + 
        current_block.header.timestamp() as i64 - epoch_now as i64;

    if cache_ttl > 0 {
        if let Err(err) = state
            .cache
            .set_ex(
                utils::CURRENT_BLOCK_NUMBER_CACHE_KEY,
                block_number,
                cache_ttl as u64,
            )
            .await
        {
            tracing::error!("Failed to cache block number: {}", err);
        }
    } else {
        tracing::warn!("Cache TTL is zero, cannot set cache for block number.");
    }

    Ok(block_number)
}

/// Fetches the current gas price from cache or provider
async fn get_gas_price(state: &AppState) -> Result<u128> {
    let mut conn = state.cache.get_conn().await?;
    
    match conn.get::<_, Option<Decimal>>(utils::GAS_PRICE_CACHE_KEY).await {
        Ok(Some(cached_gas_price)) => {
            tracing::info!("Using cached gas price");
            Ok(u128::try_from(cached_gas_price)?)
        }
        Ok(None) => {
            tracing::info!("Cached gas price not found, fetching from provider.");
            fetch_and_cache_gas_price(state).await
        }
        Err(e) => {
            tracing::error!("Failed to get cached gas price: {}", e);
            fetch_and_cache_gas_price(state).await
        }
    }
}

/// Fetches the current gas price from provider and caches it
async fn fetch_and_cache_gas_price(state: &AppState) -> Result<u128> {
    let gas_price = state.eth_provider.get_gas_price().await?;

    if let Err(err) = state
        .cache
        .set_ex(
            utils::GAS_PRICE_CACHE_KEY,
            rust_decimal::Decimal::from(gas_price),
            utils::GAS_PRICE_TTL,
        )
        .await
    {
        tracing::error!("Failed to cache gas price: {}", err);
    }

    Ok(gas_price)
}

/// Handler for getting account information
pub async fn get_account_info(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<AccountResponse>> {
    // Validate the Ethereum address format
    if !utils::is_valid_ethereum_address(&address) {
        return Err(ValidateError("Invalid Ethereum address format".to_string()).into());
    }

    // Get current block number and gas price
    let block_number = get_current_block_number(&state).await?;
    let gas_price = get_gas_price(&state).await?;

    // Get account balance
    let eth_address = address.parse()?;
    let balance = state
        .eth_provider
        .get_balance(eth_address)
        .await?
        .to_string();

    // Update database with current balance
    let balance_decimal = balance.parse()?;
    state
        .repo
        .upsert_eth_account_balance(&address, ZERO_ADDRESS, balance_decimal)
        .await?;

    Ok(Json(AccountResponse {
        address,
        current_block: block_number,
        gas_price,
        balance,
    }))
}
