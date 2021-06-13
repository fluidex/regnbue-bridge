CREATE TYPE tx_status AS ENUM('proposed', 'claimed', 'sent', 'confirmed');

CREATE TABLE faucet_tx (
    id INT CHECK (id >= 0) NOT NULL PRIMARY KEY,
    to_user INT CHECK (to_user >= 0) NOT NULL,
    asset VARCHAR(30) NOT NULL,
    amount DECIMAL(30, 8) NOT NULL
    created_time TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP
);
