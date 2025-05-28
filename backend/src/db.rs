use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};

type Result<T> = std::result::Result<T, sqlx::Error>;

/// Initializes the database connection pool.
pub async fn setup_db(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(database_url)
        .await?;
    Ok(pool)
}

/// EthAccount represents an Ethereum account in the database.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EthAccount {
    pub address: Vec<u8>,
    pub balance: rust_decimal::Decimal,
}

impl EthAccount {
    /// Retrieve one Ethereum accounts from the database.
    pub async fn get_one_by_address(pool: &PgPool, address: Vec<u8>) -> Result<Option<Self>> {
        let account = sqlx::query_as!(
            EthAccount,
            r#"
            SELECT address, balance FROM eth_accounts WHERE address = $1
            "#,
            address
        )
        .fetch_optional(pool)
        .await?;

        Ok(account)
    }

    /// Upsert an Ethereum account into the database.
    pub async fn upsert_eth_account(
        pool: &PgPool,
        address: Vec<u8>,
        balance: rust_decimal::Decimal,
    ) -> Result<()> {
        sqlx::query_as!(
            EthAccount,
            r#"
            INSERT INTO eth_accounts (address, balance)
            VALUES ($1, $2)
            ON CONFLICT (address)
            DO UPDATE SET balance = EXCLUDED.balance
            RETURNING address, balance
            "#,
            address,
            balance
        )
        .fetch_one(pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Executor;

    #[sqlx::test()]
    async fn test_upsert_eth_account(pool: PgPool) {
        let address = vec![0x1, 0x2, 0x3];
        let balance = rust_decimal::Decimal::new(100, 0);

        // Upsert the account
        EthAccount::upsert_eth_account(&pool, address.clone(), balance)
            .await
            .unwrap();

        // Retrieve the account
        let account = EthAccount::get_one_by_address(&pool, address)
            .await
            .unwrap();
        assert!(account.is_some());
        assert_eq!(account.unwrap().balance, balance);
    }

    #[sqlx::test()]
    async fn test_get_one_by_address(pool: PgPool) {
        let address = vec![0x1, 0x2, 0x3];
        let balance = rust_decimal::Decimal::new(100, 0);

        // Insert the account
        sqlx::query!(
            r#"
            INSERT INTO eth_accounts (address, balance)
            VALUES ($1, $2)
            "#,
            address,
            balance
        )
        .execute(&pool)
        .await
        .unwrap();

        // Retrieve the account
        let account = EthAccount::get_one_by_address(&pool, address)
            .await
            .unwrap();
        assert!(account.is_some());
        assert_eq!(account.unwrap().balance, balance);
    }
}
