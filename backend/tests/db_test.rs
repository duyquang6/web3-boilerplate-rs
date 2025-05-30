use alloy::primitives::address;
use backend::db::*;
use sqlx::PgPool;

#[sqlx::test()]
async fn test_upsert_eth_account_balance(pool: PgPool) {
    let address = address!("0xea921fb6d4cf7f5ced3e5a774dea51496d1ed2bf");
    let token_address = address!("0x3b3adf1422f84254b7fbb0e7ca62bd0865133fe3");
    let balance = rust_decimal::Decimal::new(100, 0);

    let repo = Repository::new(pool.clone()).await;

    repo.upsert_eth_account_balance(&address.to_string(), &token_address.to_string(), balance)
        .await
        .unwrap();

    let data = sqlx::query!(
        r#"
        SELECT balance FROM eth_account_balances
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(data.balance, balance);
}

#[sqlx::test()]
async fn test_ping(pool: PgPool) {
    let repo = Repository::new(pool.clone()).await;
    assert!(repo.ping().await.is_ok());
}
