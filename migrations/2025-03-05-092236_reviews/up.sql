CREATE TABLE reviews (
    user_id INT NOT NULL,
    item_id INT NOT NULL,
    rating INT NOT NULL,
    comment VARCHAR(255),
    PRIMARY KEY(user_id, item_id)
);
