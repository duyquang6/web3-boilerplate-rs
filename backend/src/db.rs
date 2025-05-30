// Database module for handling PostgreSQL interactions and Ethereum account data
use std::time::Duration;

use crate::error::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};

/// Configuration for database connection settings
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Maximum number of concurrent database connections
    pub max_connections: u32,
    /// Database connection URL
    pub url: String,
}

/// Repository provides methods to interact with the PostgreSQL database
/// Handles connection pooling and database operations for Ethereum account data
#[derive(Debug, Clone)]
pub struct Repository {
    /// Connection pool for managing database connections
    pool: PgPool,
}

impl Repository {
    /// Creates a new Repository instance with the provided configuration
    /// Sets up connection pooling with specified max connections and timeout
    pub async fn new_with_config(config: &Config) -> Result<Self> {
        // Set up connection pool with configured parameters
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .acquire_timeout(Duration::from_secs(1))
            .connect(&config.url)
            .await?;

        Ok(Self { pool })
    }

    /// Creates a new Repository instance with an existing connection pool
    pub async fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Runs database migrations from the migrations directory
    /// Ensures database schema is up to date
    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    /// Performs a health check on the database connection
    /// Returns Ok if the database is accessible
    pub async fn ping(&self) -> Result<()> {
        let _ = sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map(|_| ())?;
        Ok(())
    }

    /// Updates or inserts an Ethereum account balance in the database
    /// Uses upsert operation to handle both new and existing records
    /// 
    /// # Arguments
    /// * `address` - Ethereum account address
    /// * `token_address` - ERC20 token contract address
    /// * `balance` - Current token balance
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

/// Represents an Ethereum account balance record in the database
/// Stores the relationship between an account, token, and its balance
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EthAccountBalance {
    /// Ethereum account address
    pub address: String,
    /// ERC20 token contract address
    pub token_address: String,
    /// Current token balance
    pub balance: rust_decimal::Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::address;
}
