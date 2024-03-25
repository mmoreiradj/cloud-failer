CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    price REAL NOT NULL,
    image_url VARCHAR(255) NOT NULL
);
