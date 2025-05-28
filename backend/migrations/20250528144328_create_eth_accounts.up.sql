-- Add up migration script here
CREATE TABLE IF NOT EXISTS eth_accounts (
    address BYTEA PRIMARY KEY,
    balance NUMERIC NOT NULL
);
