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
pub mod db;

async fn setup_app() -> Router {
    let eth_provider = eth::setup_provider(&CONFIG.eth_rpc_url)
        .await
        .expect("setup eth_provider failed");

    let repo = db::Repository::new_with_config(&CONFIG.database)
        .await
        .expect("setup repository failed");

    repo.run_migrations().await.expect("run migrations failed");

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

    Router::new()
        .route("/ping", get(async || -> Result<()> { Ok(()) }))
        .route("/health", get(handlers::health::healthcheck))
        .nest("/v1/public/eth/accounts", eth_accounts_router)
        .with_state(app_state)
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

    let app = setup_app().await;

    let serve_addr = format!("{}:{}", CONFIG.host, CONFIG.port);
    let listener = TcpListener::bind(serve_addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use axum::body::Body;
//     use axum::http::{Request, StatusCode};
//     use axum_test::TestServer;
//     use serde_json::Value;

//     #[tokio::test]
//     async fn test_get_account_endpoint() {
//         let app = setup_app().await;

//         let server = TestServer::new(app).expect("Failed to create test server");

//         let address = "0x";
//     }
// }
