-- Your SQL goes here
CREATE TABLE goods (
    id SERIAL PRIMARY KEY,
    code VARCHAR(10) DEFAULT LPAD(CAST(FLOOR(1 + random() * 9)::INT * 1000000000 + FLOOR(random() * 1000000000)::INT AS VARCHAR), 10, '0') NOT NULL UNIQUE,
    name VARCHAR NOT NULL,
    currency VARCHAR(3) NOT NULL,
    price FLOAT4 NOT NULL,
    quantity FLOAT4 NOT NULL DEFAULT 0,
    unit VARCHAR(50) NULL,
    created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_by VARCHAR(10) NULL,
    updated_by VARCHAR(10) NULL
);

INSERT INTO goods (code, name, currency, price, quantity, unit, created_by, updated_by)
VALUES
    ('0000000010', 'Organic Almonds', 'USD', 15.49, 50.00, 'kg', 'admin001', 'admin001'),
    ('0000000020', 'Stainless Steel Water Bottle', 'USD', 23.99, 150.00, 'pcs', 'admin001', 'admin001'),
    ('0000000030', 'Bluetooth Headphones', 'USD', 89.99, 75.00, 'pcs', 'admin001', 'admin001'),
    ('0000000040', 'Wooden Cutting Board', 'USD', 29.99, 120.00, 'pcs', 'admin001', 'admin001'),
    ('0000000050', 'Eco-Friendly Notebook', 'USD', 9.99, 300.00, 'pads', 'admin001', 'admin001'),
    ('0000000060', 'Yoga Mat', 'USD', 35.99, 80.00, 'rolls', 'admin001', 'admin001'),
    ('0000000070', 'Ceramic Mug', 'USD', 12.49, 200.00, 'units', 'admin001', 'admin001'),
    ('0000000080', 'Handmade Soap', 'USD', 6.99, 500.00, 'bars', 'admin001', 'admin001'),
    ('0000000090', 'Silicone Baking Mat', 'USD', 18.75, 40.00, 'sheets', 'admin001', 'admin001'),
    ('0000000100', 'Portable Charger', 'USD', 24.99, 60.00, 'units', 'admin001', 'admin001'),
    ('0000000110', 'Desk Organizer', 'USD', 15.99, 90.00, 'sets', 'admin001', 'admin001'),
    ('0000000120', 'Travel Pillow', 'USD', 22.50, 70.00, 'pieces', 'admin001', 'admin001');
