use alloy::providers::Provider;
use axum::extract::State;
use redis::AsyncCommands;

use crate::error::Result;
use crate::state::AppState;

/// Performs a health check on all system components
/// Checks:
/// - Database connection
/// - Ethereum provider connection
/// - Redis cache connection
pub async fn healthcheck(State(state): State<AppState>) -> Result<()> {
    // Check database connection
    state.repo.ping().await?;
    tracing::debug!("Database health check passed");

    // Check Ethereum provider connection
    state.eth_provider.get_chain_id().await?;
    tracing::debug!("Ethereum provider health check passed");

    // Check Redis cache connection
    let _: () = state.cache.get_conn().await?.ping().await?;
    tracing::debug!("Redis cache health check passed");

    tracing::info!("All health checks passed successfully");
    Ok(())
}
