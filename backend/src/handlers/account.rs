use alloy::providers::Provider;
use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;

use crate::error::{Result, ValidateError};
use crate::eth::ZERO_ADDRESS;
use crate::state::AppState;

use super::utils;

/// Response structure for account information
#[derive(Serialize)]
pub struct AccountResponse {
    address: String,
    balance: String,
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

    Ok(Json(AccountResponse { address, balance }))
}
