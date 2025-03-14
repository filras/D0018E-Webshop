CREATE TABLE comments (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id),
    review_id INT NOT NULL REFERENCES reviews(id),
    comment_id INT REFERENCES comments(id) ON DELETE CASCADE,
    comment VARCHAR(255) NOT NULL
);
