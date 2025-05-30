use std::{any::Any, sync::Arc};

use alloy::providers::DynProvider;
use sqlx::PgPool;
use tokio::sync::Mutex;

// the application state
#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
    pub eth_provider: DynProvider,
}
