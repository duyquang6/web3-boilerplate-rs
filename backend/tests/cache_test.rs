use std::time::Duration;
use tokio;
use redis::AsyncCommands;

use backend::cache::{Config, DistCache};

// Helper function to create a test cache instance
fn create_test_cache() -> DistCache {
    let config = Config {
        redis_url: "redis://127.0.0.1:6379".to_string(),
        connect_timeout: 5,
    };
    DistCache::new(&config)
}

#[tokio::test]
async fn test_cache_connection() {
    let cache = create_test_cache();
    let conn = cache.get_conn().await;
    assert!(conn.is_ok(), "Failed to connect to Redis");
}

#[tokio::test]
async fn test_set_and_get() {
    let cache = create_test_cache();
    let mut conn = cache.get_conn().await.unwrap();

    // Test string value
    let key = "test:string";
    let value = "test_value";
    cache.set_ex(key, value, 60).await.unwrap();
    let result: String = conn.get(key).await.unwrap();
    assert_eq!(result, value);

    // Test numeric value
    let key = "test:number";
    let value = 42i64;
    cache.set_ex(key, value, 60).await.unwrap();
    let result: i64 = conn.get(key).await.unwrap();
    assert_eq!(result, value);
}

#[tokio::test]
async fn test_ttl() {
    let cache = create_test_cache();
    let mut conn = cache.get_conn().await.unwrap();

    let key = "test:ttl";
    let value = "expiring_value";
    let ttl = 2; // 2 seconds

    cache.set_ex(key, value, ttl).await.unwrap();
    
    // Value should exist immediately
    let result: String = conn.get(key).await.unwrap();
    assert_eq!(result, value);

    // Wait for TTL to expire
    tokio::time::sleep(Duration::from_secs(ttl + 1)).await;

    // Value should be gone
    let result: Option<String> = conn.get(key).await.unwrap();
    assert!(result.is_none(), "Value should have expired");
}

#[tokio::test]
async fn test_concurrent_operations() {
    let cache = create_test_cache();
    let mut handles = vec![];

    // Spawn multiple tasks that set different values
    for i in 0..5 {
        let cache = cache.clone();
        let handle = tokio::spawn(async move {
            let key = format!("test:concurrent:{}", i);
            let value = format!("value_{}", i);
            cache.set_ex(&key, &value, 60).await.unwrap();
            
            let mut conn = cache.get_conn().await.unwrap();
            let result: String = conn.get(&key).await.unwrap();
            assert_eq!(result, value);
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::test]
async fn test_error_handling() {
    let cache = create_test_cache();
    let mut conn = cache.get_conn().await.unwrap();

    // Test invalid key type
    let key = "test:invalid";
    let value = "test_value";
    cache.set_ex(key, value, 60).await.unwrap();
    
    // Try to get as wrong type
    let result: Result<i64, _> = conn.get(key).await;
    assert!(result.is_err(), "Should fail when getting string as i64");
}
