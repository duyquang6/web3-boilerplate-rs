use std::convert::TryFrom;
use std::time::SystemTime;

use alloy::{consensus::BlockHeader, providers::Provider};
use anyhow::anyhow;
use axum::{
    Json,
    extract::{ State},
};
use redis::AsyncCommands;
use rust_decimal::Decimal;
use serde::Serialize;

use crate::error::{NotFoundError, Result};
use crate::state::AppState;

use super::utils;

/// Fetches the current block number from cache or provider
async fn get_current_block_number(state: &AppState) -> Result<u64> {
    let mut conn = state.cache.get_conn().await?;

    match conn.get(utils::CURRENT_BLOCK_NUMBER_CACHE_KEY).await {
        Ok(Some(block_number)) => {
            tracing::info!("Using cached block number: {}", block_number);
            Ok(block_number)
        }
        Ok(None) => {
            tracing::info!(
                "Cached block number not found, fetching latest block header from provider."
            );
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

    let cache_ttl = utils::BLOCK_MINE_DURATION as i64 + current_block.header.timestamp() as i64
        - epoch_now as i64;

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

    match conn
        .get::<_, Option<Decimal>>(utils::GAS_PRICE_CACHE_KEY)
        .await
    {
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

/// Response structure for blockchain misc information
#[derive(Serialize)]
pub struct BlockchainMiscResponse {
    current_block: u64,
    gas_price: u128,
}

/// Handler for getting blockchain misc information
pub async fn get_blockchain_misc(
    State(state): State<AppState>,
) -> Result<Json<BlockchainMiscResponse>> {
    // Get current block number and gas price
    let block_number = get_current_block_number(&state).await?;
    let gas_price = get_gas_price(&state).await?;

    Ok(Json(BlockchainMiscResponse {
        current_block: block_number,
        gas_price,
    }))
}
