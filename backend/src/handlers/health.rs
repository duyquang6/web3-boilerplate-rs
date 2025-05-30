use alloy::providers::Provider;
use axum::extract::State;
use redis::AsyncCommands;

use crate::error::Result;
use crate::state::AppState;

pub async fn healthcheck(State(state): State<AppState>) -> Result<()> {
    state.repo.ping().await?;
    state.eth_provider.get_chain_id().await?;
    let _: () = state.cache.get_conn().await?.ping().await?;

    tracing::info!("Health check passed");

    Ok(())
}
