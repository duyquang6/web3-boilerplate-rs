use std::time::SystemTime;

use alloy::{consensus::BlockHeader, providers::Provider};
use anyhow::anyhow;
use axum::{
    Json,
    extract::{Path, State},
};
use redis::AsyncCommands;
use serde::Serialize;

use crate::error::{NotFoundError, Result, ValidateError};
use crate::{eth::ZERO_ADDRESS, state::AppState};

#[derive(Serialize)]
pub struct AccountResponse {
    address: String,
    current_block: u64,
    gas_price: u128,
    balance: String,
}

/// Hard coded since network is not change frequently
/// Consider move to config file if change to other kind of network
const CURRENT_BLOCK_NUMBER_CACHE_KEY: &str = "current_block:number";
const BLOCK_MINE_DURATION: u64 = 12;

const GAS_PRICE_CACHE_KEY: &str = "gas_price";
const GAS_PRICE_TTL: u64 = 1; // 1 seconds

pub async fn get_account_info(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<AccountResponse>> {
    // Validate the Ethereum address format
    if !address.starts_with("0x") || address.len() != 42 {
        return Err(ValidateError("Invalid Ethereum address format".to_string()).into());
    }
    let mut block_number = 0u64;

    // init redis conn
    let mut conn = state.cache.get_conn().await?;
    // try to get from cache
    let mut should_fetch_latest_block = false;

    let cache_block_number = conn.get(CURRENT_BLOCK_NUMBER_CACHE_KEY).await;

    match cache_block_number {
        Ok(Some(cache_block_number)) => {
            block_number = cache_block_number;
            tracing::info!("Using cached block number: {}", block_number);
        }
        Ok(None) => {
            tracing::info!(
                "Cached block number not found, fetching latest block header from provider."
            );
            should_fetch_latest_block = true;
        }
        Err(e) => {
            // fallback to fetch latest block header from provider
            tracing::error!("Failed to get cached block number: {}", e);
            should_fetch_latest_block = true;
        }
    }

    if should_fetch_latest_block {
        let current_block = state
            .eth_provider
            .get_block_by_number(alloy::eips::BlockNumberOrTag::Latest)
            .await?
            .ok_or_else(|| NotFoundError("Latest block not found".to_string()))?;

        block_number = current_block.header.number;

        let epoch_now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| anyhow!("Failed to get current time"))?
            .as_secs();
        let cache_ttl =
            BLOCK_MINE_DURATION as i64 + current_block.header.timestamp() as i64 - epoch_now as i64;

        if cache_ttl <= 0 {
            tracing::warn!("Cache TTL is zero, cannot set cache for block number.");
        } else if let Err(err) = state
            .cache
            .set_ex(
                CURRENT_BLOCK_NUMBER_CACHE_KEY,
                block_number,
                cache_ttl as u64,
            )
            .await
        {
            tracing::error!("Failed to cache block number: {}", err);
        }
    }

    // Fetch gas price from cache or provider
    let cache_gas_price: redis::RedisResult<Option<rust_decimal::Decimal>> =
        conn.get(GAS_PRICE_CACHE_KEY).await;

    let gas_price = if let Ok(Some(cached_gas_price)) = cache_gas_price {
        tracing::info!("Using cached gas price");
        u128::try_from(cached_gas_price)?
    } else {
        tracing::info!("Cached gas price not found, fetching from provider.");

        let gas_price = state.eth_provider.get_gas_price().await?;

        // set cache for gas price
        if let Err(err) = state
            .cache
            .set_ex(
                GAS_PRICE_CACHE_KEY,
                rust_decimal::Decimal::from(gas_price),
                GAS_PRICE_TTL,
            )
            .await
        {
            tracing::error!("Failed to cache gas price: {}", err);
        }

        gas_price
    };

    let eth_address = address.parse()?;
    let balance = state
        .eth_provider
        .get_balance(eth_address)
        .await?
        .to_string();

    let balance_decimal = balance.parse()?;
    state
        .repo
        .upsert_eth_account_balance(&address.to_string(), ZERO_ADDRESS, balance_decimal)
        .await?;

    Ok(Json(AccountResponse {
        address,
        current_block: block_number,
        gas_price,
        balance,
    }))
}
