use axum::{
    http::StatusCode, routing::get, Router
};
use axum_test::TestServer;
use serde_json::Value;

use backend::{
    cache::DistCache, config::CONFIG, db::Repository, eth::setup_provider, handlers::{
        account::get_account_info,
        erc20::get_account_erc20,
        health::healthcheck, misc::get_blockchain_misc,
    }, state::AppState
};

// Helper function to create a test router
async fn create_test_router() -> Router {
    let eth_provider = setup_provider(&CONFIG.eth_rpc_url)
        .await
        .expect("Failed to setup eth provider");

    let repo = Repository::new_with_config(&CONFIG.database)
        .await
        .expect("Failed to setup repository");

    let cache = DistCache::new(&CONFIG.cache);
    let app_state = AppState {
        repo,
        eth_provider,
        cache,
    };

    Router::new()
        .route("/health", get(healthcheck))
        .route("/v1/public/eth/accounts/{address}", get(get_account_info))
        .route(
            "/v1/public/eth/accounts/{address}/erc20/{token_address}",
            get(get_account_erc20),
        )
        .route("/v1/public/eth/misc", get(get_blockchain_misc))
        .with_state(app_state)
}

#[tokio::test]
async fn test_health_check() {
    let app = create_test_router().await;
    let server = TestServer::new(app).unwrap();

    let response = server.get("/health").await;
    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_get_account_info_invalid_address() {
    let app = create_test_router().await;
    let server = TestServer::new(app).unwrap();

    let response = server
        .get("/v1/public/eth/accounts/invalid_address")
        .await;

    assert_eq!(response.status_code(), 400);
    let body: Value = response.json();
    assert!(body["error_msg"].as_str().unwrap().contains("Invalid Ethereum address format"));
}

#[tokio::test]
async fn test_get_account_info_valid_address() {
    let app = create_test_router().await;
    let server = TestServer::new(app).unwrap();

    // Using a known test address
    let test_address = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
    let response = server
        .get(&format!("/v1/public/eth/accounts/{}", test_address))
        .await;

    assert_eq!(response.status_code(), 200);
    let body: Value = response.json();
    assert_eq!(body["address"], test_address);
    assert!(body["balance"].is_string());
}

#[tokio::test]
async fn test_get_erc20_balance_invalid_addresses() {
    let app = create_test_router().await;
    let server = TestServer::new(app).unwrap();

    // Test invalid account address
    let response = server
        .get("/v1/public/eth/accounts/invalid_address/erc20/0x123")
        .await;
    assert_eq!(response.status_code(), 400);

    // Test invalid token address
    let response = server
        .get("/v1/public/eth/accounts/0x123/erc20/invalid_token")
        .await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_get_erc20_balance_valid_addresses() {
    let app = create_test_router().await;
    let server = TestServer::new(app).unwrap();

    // Using known test addresses
    let account_address = "0xd27de11aaacd14c62fe689d214a67e9385e6f60c";
    let token_address = "0xab809CB0aB6669d51f6189432f751f1a916a10cd"; 

    let response = server
        .get(&format!(
            "/v1/public/eth/accounts/{}/erc20/{}",
            account_address, token_address
        ))
        .await;

    assert_eq!(response.status_code(), 200);
    let body: Value = response.json();
    assert_eq!(body["address"].as_str().unwrap().to_lowercase(), account_address);
    assert_eq!(body["token_address"], token_address);
    assert!(body["balance"].is_string());
}

#[tokio::test]
async fn test_get_account_endpoint() {
    let app = create_test_router().await;
    let server = TestServer::new(app).expect("Failed to create test server");

    // Test with invalid address
    let response = server
        .get("/v1/public/eth/accounts/0xinvalid")
        .await;
    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

    // Test with valid address
    let response = server
        .get("/v1/public/eth/accounts/0x742d35Cc6634C0532925a3b844Bc454e4438f44e")
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body.get("address").is_some());
    assert!(body.get("balance").is_some());
}

#[tokio::test]
async fn test_get_blockchain_misc_endpoint() {
    let app = create_test_router().await;
    let server = TestServer::new(app).expect("Failed to create test server");

    let response = server
        .get("/v1/public/eth/misc")
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body.get("current_block").is_some());
    assert!(body.get("gas_price").is_some());
}
