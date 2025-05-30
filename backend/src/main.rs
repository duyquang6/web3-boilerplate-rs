use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tokio::{net::TcpListener, sync::Mutex};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::{any::Any, num::NonZeroUsize, sync::Arc, time::Duration};

mod error;
use error::Result;

mod config;
use config::CONFIG;

mod eth;

mod handlers;
mod state;

mod db;

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

    let eth_provider = eth::setup_provider(&CONFIG.eth_rpc_url)
        .await
        .expect("setup eth_provider failed");

    let app_state = AppState {
        pg_pool: pool,
        eth_provider,
    };

    let eth_accounts_router = Router::new()
        .route("/{address}", get(handlers::account::get_account_info))
        .route(
            "/{address}/erc20/{token_address}",
            get(handlers::erc20::get_account_erc20),
        )
        .with_state(app_state);

    let app = Router::new()
        .route("/ping", get(async || -> Result<()> { Ok(()) }))
        .nest("/v1/public/eth/accounts", eth_accounts_router);

    let serve_addr = format!("{}:{}", CONFIG.host, CONFIG.port);
    let listener = TcpListener::bind(serve_addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
