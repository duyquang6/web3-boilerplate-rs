use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;

use crate::state::AppState;
use crate::{db::EthAccountBalance, error::ValidateError};
use crate::{error::Result, eth::IERC20Instance};

#[derive(Serialize)]
pub struct Erc20TokenResponse {
    address: String,
    token_address: String,
    balance: String,
}

pub async fn get_account_erc20(
    Path((address, token_address)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Json<Erc20TokenResponse>> {
    // Validate the Ethereum address format
    if !address.starts_with("0x") || address.len() != 42 {
        return Err(ValidateError("Invalid Ethereum address format".to_string()).into());
    }
    if !token_address.starts_with("0x") || token_address.len() != 42 {
        return Err(ValidateError("Invalid token address format".to_string()).into());
    }

    let token_address = token_address.parse()?;
    let contract = IERC20Instance::new(token_address, state.eth_provider);

    let address = address.parse()?;
    let erc20_balance = contract.balanceOf(address).call().await?.to_string();

    // insert db
    let erc20_balance_decimal = erc20_balance.parse()?;
    EthAccountBalance::upsert(
        &state.pg_pool,
        &address.to_string().to_lowercase(),
        &token_address.to_string().to_lowercase(),
        erc20_balance_decimal,
    )
    .await?;

    Ok(Json(Erc20TokenResponse {
        address: address.to_string(),
        token_address: token_address.to_string(),
        balance: erc20_balance.to_string(),
    }))
}
