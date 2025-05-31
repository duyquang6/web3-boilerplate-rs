// Main application entry point for the Ethereum account information service
use axum::{Router, routing::get};
use state::AppState;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Module imports for error handling, configuration, and core functionality
mod error;
use error::Result;

mod config;
use config::CONFIG;

mod eth;

mod handlers;
mod state;

mod cache;
pub mod db;

/// Sets up the application router with all necessary routes and middleware
/// Initializes the Ethereum provider, database repository, and cache
async fn setup_app() -> Router {
    // Initialize Ethereum provider with configured RPC URL
    let eth_provider = eth::setup_provider(&CONFIG.eth_rpc_url)
        .await
        .expect("setup eth_provider failed");

    // Initialize database repository with configuration
    let repo = db::Repository::new_with_config(&CONFIG.database)
        .await
        .expect("setup repository failed");

    // Run database migrations
    repo.run_migrations().await.expect("run migrations failed");

    // Initialize distributed cache
    let dist_cache = cache::DistCache::new(&CONFIG.cache);
    
    // Create application state with all dependencies
    let app_state = AppState {
        repo,
        eth_provider,
        cache: dist_cache,
    };

    // Set up Ethereum accounts router with endpoints
    let eth_accounts_router = Router::new()
        .route("/{address}", get(handlers::account::get_account_info))
        .route(
            "/{address}/erc20/{token_address}",
            get(handlers::erc20::get_account_erc20),
        );

    // Create main router with all routes and middleware
    Router::new()
        .route("/ping", get(async || -> Result<()> { Ok(()) }))
        .route("/health", get(handlers::health::healthcheck))
        .nest("/v1/public/eth/accounts", eth_accounts_router)
        .route("/v1/public/eth/misc", get(handlers::misc::get_blockchain_misc))
        .with_state(app_state)
}

/// Main entry point of the application
#[tokio::main]
async fn main() {
    // Initialize logging with environment-based configuration
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Set up the application router
    let app = setup_app().await;

    // Configure and start the HTTP server
    let serve_addr = format!("{}:{}", CONFIG.host, CONFIG.port);
    let listener = TcpListener::bind(serve_addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
