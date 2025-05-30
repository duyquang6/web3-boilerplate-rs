use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

type Result<T> = std::result::Result<T, sqlx::Error>;

/// EthAccount represents an Ethereum account in the database.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EthAccountBalance {
    pub address: String,
    pub token_address: String,
    pub balance: rust_decimal::Decimal,
}

impl EthAccountBalance {
    /// Upserts an Ethereum account balance into the database.
    pub async fn upsert(
        pool: &PgPool,
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
        .execute(pool)
        .await?;

        Ok(())
    }
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

        // Upsert the account
        EthAccountBalance::upsert(
            &pool,
            &address.to_string(),
            &token_address.to_string(),
            balance,
        )
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
