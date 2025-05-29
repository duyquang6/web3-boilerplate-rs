use alloy::{
    providers::{DynProvider, Provider, ProviderBuilder},
    sol_types::sol,
};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{self, IntoResponse},
    routing::get,
};
use config::{Environment, File};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::{sync::LazyLock, time::Duration};

/// AppConfig define config
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database: Database,
    pub eth_rpc_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub max_connections: u32,
    pub url: String,
}

static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| {
    let cfg = config::Config::builder()
        .add_source(File::with_name("config/local")) // `config/local.toml`
        .set_override(
            "database.url",
            std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:123abc@localhost".to_string()),
        )
        .unwrap()
        .add_source(Environment::with_prefix("APP").separator("__")) // APP__DATABASE__URL
        .build()
        .expect("parse config error");

    cfg.try_deserialize().expect("deserialize failed")
});

async fn setup_eth_provider(rpc_url: &str) -> Result<DynProvider> {
    let rpc_url = rpc_url.parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    let chain_id = provider.get_chain_id().await?;
    tracing::info!("Success connect to chain_id {} network", chain_id);

    Ok(provider.erased())
}

/// Result
type Result<T> = std::result::Result<T, AppError>;
#[derive(Debug)]
struct AppError(anyhow::Error);
// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> response::Response {
        let json_response = json!({"error_msg": self.0.to_string()});
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(CONFIG.database.max_connections)
        .acquire_timeout(Duration::from_secs(1))
        .connect(&CONFIG.database.url)
        .await
        .expect("can't connect to database");

    let eth_provider = setup_eth_provider(&CONFIG.eth_rpc_url)
        .await
        .expect("setup eth_provider failed");

    let app_state = AppState {
        pg_pool: pool,
        eth_provider,
    };

    let eth_accounts_router = Router::new()
        .route("/{address}", get(get_account_info))
        .route("/{address}/erc20/{token_address}", get(get_account_erc20))
        .with_state(app_state);

    let app = Router::new()
        .route("/ping", get(async || -> Result<()> { Ok(()) }))
        .nest("/v1/public/eth/accounts", eth_accounts_router);

    let serve_addr = format!("{}:{}", CONFIG.host, CONFIG.port);
    let listener = TcpListener::bind(serve_addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// the application state
#[derive(Clone)]
struct AppState {
    pg_pool: PgPool,
    eth_provider: DynProvider,
}

#[derive(Serialize)]
struct AccountResponse {
    address: String,
    current_block: u64,
    gas_price: u128,
    balance: String,
}

async fn get_account_info(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<AccountResponse>> {
    let current_block = state.eth_provider.get_block_number().await?;
    let gas_price = state.eth_provider.get_gas_price().await?;
    let eth_address = address.parse()?;
    let balance = state
        .eth_provider
        .get_balance(eth_address)
        .await?
        .to_string();

    Ok(Json(AccountResponse {
        address,
        current_block,
        gas_price,
        balance,
    }))
}

/// ERC20 Handlers interact with ERC20 contracts (balances, minting,...)
sol!(
    #[sol(rpc)]
    IERC20,
    "abi/IERC20.json"
);

use IERC20::IERC20Instance;

#[derive(Serialize)]
struct Erc20TokenResponse {
    address: String,
    token_address: String,
    balance: String,
}

async fn get_account_erc20(
    Path((address, token_address)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Json<Erc20TokenResponse>> {
    let token_address = token_address.parse()?;
    let contract = IERC20Instance::new(token_address, state.eth_provider);

    let address = address.parse()?;
    let erc20_balance = contract.balanceOf(address).call().await?;

    Ok(Json(Erc20TokenResponse {
        address: address.to_string(),
        token_address: token_address.to_string(),
        balance: erc20_balance.to_string(),
    }))
}
