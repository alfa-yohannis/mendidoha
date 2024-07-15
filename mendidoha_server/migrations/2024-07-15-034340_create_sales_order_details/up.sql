-- Your SQL goes here
CREATE TABLE sales_order_details (
    id SERIAL PRIMARY KEY,
    order_id INTEGER NOT NULL REFERENCES sales_orders(id),
    item_id INTEGER NOT NULL,
    item_type VARCHAR NOT NULL CHECK (item_type IN ('good', 'service')),
    quantity DECIMAL(10, 2) NOT NULL DEFAULT 0,
    unit VARCHAR(50) NULL,
    unit_price NUMERIC(10, 2) NOT NULL,
    created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_by VARCHAR(10) NULL,
    updated_by VARCHAR(10) NULL
);
