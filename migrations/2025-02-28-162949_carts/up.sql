CREATE TABLE cart_items (
    user_id INT NOT NULL REFERENCES users(id),
    item_id iNT NOT NULL REFERENCES items(id),
    amount INT NOT NULL,
    PRIMARY KEY(user_id, item_id)
    
);
