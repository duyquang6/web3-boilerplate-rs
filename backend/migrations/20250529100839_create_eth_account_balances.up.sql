-- Add up migration script here
CREATE TABLE IF NOT EXISTS eth_account_balances (
        address CHAR(42) NOT NULL,
        token_address CHAR(42) NOT NULL,
        balance NUMERIC NOT NULL,
        PRIMARY KEY (address, token_address)
    );