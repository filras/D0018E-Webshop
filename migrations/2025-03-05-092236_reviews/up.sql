CREATE TABLE reviews (
    user_id INT NOT NULL REFERENCES users(id),
    item_id INT NOT NULL REFERENCES items(id),
    rating INT NOT NULL,
    comment VARCHAR(255),
    PRIMARY KEY (user_id, item_id)
);
