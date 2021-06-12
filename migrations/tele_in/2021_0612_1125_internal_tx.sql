CREATE TYPE tx_status AS ENUM('proposed', 'sent', 'confirmed');

CREATE TABLE internal_tx (
    id TIMESTAMP(0) NOT NULL, -- consistent to dingir-exchange "internal_tx" table's time column, as tx_id.
    to_user INT CHECK (to_user >= 0) NOT NULL,
    asset VARCHAR(30) NOT NULL,
    amount DECIMAL(30, 8) NOT NULL
    created_time TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);
