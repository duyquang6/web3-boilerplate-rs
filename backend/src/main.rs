use axum::{Router, routing::get};
use state::AppState;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
use error::Result;

mod config;
use config::CONFIG;

mod eth;

mod handlers;
mod state;

mod cache;
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

    let eth_provider = eth::setup_provider(&CONFIG.eth_rpc_url)
        .await
        .expect("setup eth_provider failed");

    let repo = db::Repository::new_with_config(&CONFIG.database)
        .await
        .expect("setup repository failed");

    let dist_cache = cache::DistCache::new(&CONFIG.cache);
    let app_state = AppState {
        repo,
        eth_provider,
        cache: dist_cache,
    };

    let eth_accounts_router = Router::new()
        .route("/{address}", get(handlers::account::get_account_info))
        .route(
            "/{address}/erc20/{token_address}",
            get(handlers::erc20::get_account_erc20),
        );

    let app = Router::new()
        .route("/ping", get(async || -> Result<()> { Ok(()) }))
        .route("/health", get(handlers::health::healthcheck))
        .nest("/v1/public/eth/accounts", eth_accounts_router)
        .with_state(app_state);

    let serve_addr = format!("{}:{}", CONFIG.host, CONFIG.port);
    let listener = TcpListener::bind(serve_addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
