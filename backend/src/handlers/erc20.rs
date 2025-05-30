use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;

use crate::error::ValidateError;
use crate::state::AppState;
use crate::{error::Result, eth::IERC20Instance};

use super::utils;

/// Response structure for ERC20 token balance information
#[derive(Serialize)]
pub struct Erc20TokenResponse {
    address: String,
    token_address: String,
    balance: String,
}

/// Handler for getting ERC20 token balance
pub async fn get_account_erc20(
    Path((address, token_address)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Json<Erc20TokenResponse>> {
    // Validate Ethereum addresses
    if !utils::is_valid_ethereum_address(&address) {
        return Err(ValidateError("Invalid Ethereum address format".to_string()).into());
    }
    if !utils::is_valid_ethereum_address(&token_address) {
        return Err(ValidateError("Invalid token address format".to_string()).into());
    }

    // Parse addresses
    let token_address = token_address.parse()?;
    let address = address.parse()?;

    // Get token balance
    let contract = IERC20Instance::new(token_address, state.eth_provider);
    let erc20_balance = contract.balanceOf(address).call().await?;

    // Update database with current balance
    let erc20_balance_decimal = erc20_balance.to_string().parse()?;
    state
        .repo
        .upsert_eth_account_balance(
            &address.to_string(),
            &token_address.to_string(),
            erc20_balance_decimal,
        )
        .await?;

    Ok(Json(Erc20TokenResponse {
        address: address.to_string(),
        token_address: token_address.to_string(),
        balance: erc20_balance.to_string(),
    }))
}
