CREATE TYPE tx_status AS ENUM('proposed', 'sent', 'confirmed');

CREATE TABLE internal_tx (
    id VARCHAR(30) NOT NULL,
    -- original_id,
    -- to_user,
    -- asset,
    -- amount,
    created_time TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_time TIMESTAMP(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id) -- serial?
);
