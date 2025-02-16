CREATE TABLE items (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    price INTEGER NOT NULL,
    in_stock INTEGER NOT NULL,
    average_rating FLOAT,
    discounted_price INTEGER
);
