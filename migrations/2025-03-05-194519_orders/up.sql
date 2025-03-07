CREATE TABLE orders(
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    address VARCHAR(255) NOT NULL,
    co VARCHAR(255),
    zipcode VARCHAR(255) NOT NULL,
    country VARCHAR(255) NOT NULL,
    total INTEGER NOT NULL,
    comment VARCHAR(255),
    payment_completed BOOLEAN NOT NULL
);
