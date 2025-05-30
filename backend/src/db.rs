use std::time::Duration;

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};

type Result<T> = std::result::Result<T, sqlx::Error>;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub max_connections: u32,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub async fn new_with_config(config: &Config) -> Result<Self> {
        // set up connection pool
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .acquire_timeout(Duration::from_secs(1))
            .connect(&config.url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn ping(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map(|_| ())
    }

    /// Upserts an Ethereum account balance into the database.
    pub async fn upsert_eth_account_balance(
        &self,
        address: &str,
        token_address: &str,
        balance: rust_decimal::Decimal,
    ) -> Result<()> {
        sqlx::query_as!(
            EthAccountBalance,
            r#"
            INSERT INTO eth_account_balances (address, token_address, balance)
            VALUES ($1, $2, $3)
            ON CONFLICT (address, token_address)
            DO UPDATE SET balance = EXCLUDED.balance
            "#,
            address.to_lowercase(),
            token_address.to_lowercase(),
            balance
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

/// EthAccount represents an Ethereum account in the database.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EthAccountBalance {
    pub address: String,
    pub token_address: String,
    pub balance: rust_decimal::Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::address;

    #[sqlx::test()]
    async fn test_upsert_eth_account_balance(pool: PgPool) {
        let address = address!("0xea921fb6d4cf7f5ced3e5a774dea51496d1ed2bf");
        let token_address = address!("0x3b3adf1422f84254b7fbb0e7ca62bd0865133fe3");
        let balance = rust_decimal::Decimal::new(100, 0);

        let repo = Repository::new(pool.clone()).await;

        // Upsert the account
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
}
