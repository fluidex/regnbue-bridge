CREATE TYPE tx_status AS ENUM('proposed', 'claimed', 'sent', 'confirmed');

CREATE TABLE faucet_tx (
    id SERIAL PRIMARY KEY,
    to_user INT CHECK (to_user >= 0) NOT NULL,
    asset VARCHAR(30) NOT NULL,
    amount DECIMAL(30, 8) NOT NULL,
    status tx_status NOT NULL DEFAULT 'proposed',
    created_time TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_timestamp BEFORE UPDATE
ON faucet_tx FOR EACH ROW EXECUTE PROCEDURE
update_timestamp();