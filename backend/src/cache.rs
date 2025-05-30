use redis::{self, AsyncCommands, AsyncConnectionConfig, ToRedisArgs, aio::MultiplexedConnection};
use serde::Deserialize;

use crate::error::Result;
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub redis_url: String,
    pub connect_timeout: u64,
}

// Cache for distributed data using Redis
// This cache can be used to store and retrieve data across multiple instances of the application
// It is useful for caching frequently accessed data, such as block headers or gas prices
#[derive(Clone)]
pub struct DistCache {
    pub client: redis::Client,
    connect_timeout: u64,
}

impl DistCache {
    pub fn new(config: &Config) -> Self {
        let client = redis::Client::open(config.redis_url.clone()).expect("Invalid Redis URL");
        Self {
            client,
            connect_timeout: config.connect_timeout,
        }
    }

    pub async fn get_conn(&self) -> Result<MultiplexedConnection> {
        let conn = self
            .client
            .get_multiplexed_async_connection_with_config(
                &AsyncConnectionConfig::default()
                    .set_connection_timeout(std::time::Duration::from_secs(self.connect_timeout)),
            )
            .await?;

        Ok(conn)
    }

    pub async fn set_ex<T>(&self, key: &str, value: T, ttl: u64) -> Result<()>
    where
        T: ToRedisArgs + Send + Sync,
    {
        let mut conn: MultiplexedConnection = self.get_conn().await?;
        let _: () = conn.set_ex(key, value, ttl).await?;
        Ok(())
    }
}
