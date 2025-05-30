// Utility module for common functions and constants

use alloy::primitives::Address;
use anyhow::Result;

/// Cache keys and TTLs
pub const CURRENT_BLOCK_NUMBER_CACHE_KEY: &str = "current_block:number";
pub const GAS_PRICE_CACHE_KEY: &str = "gas_price";
pub const BLOCK_MINE_DURATION: u64 = 12;
pub const GAS_PRICE_TTL: u64 = 1; // 1 second

/// Validates an Ethereum address format
/// Returns true if the address is valid, false otherwise
pub fn is_valid_ethereum_address(address: &str) -> bool {
    address.starts_with("0x") && address.len() == 42
}
