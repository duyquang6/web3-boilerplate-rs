use alloy::providers::DynProvider;

use crate::{cache::DistCache, db::Repository};

// the application state
#[derive(Clone)]
pub struct AppState {
    pub repo: Repository,
    pub eth_provider: DynProvider,
    pub cache: DistCache,
}
