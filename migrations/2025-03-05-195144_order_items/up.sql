CREATE TABLE order_items(
    order_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    total INTEGER NOT NULL,
    PRIMARY KEY(order_id, item_id)
);
