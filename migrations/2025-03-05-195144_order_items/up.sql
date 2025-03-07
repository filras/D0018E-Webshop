CREATE TABLE order_items(
    order_id INTEGER NOT NULL REFERENCES orders(id),
    item_id INTEGER NOT NULL REFERENCES items(id),
    amount INTEGER NOT NULL,
    total INTEGER NOT NULL,
    PRIMARY KEY(order_id, item_id)
);
