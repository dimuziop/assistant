-- Your SQL goes here
CREATE TABLE credentials
(
    id         VARCHAR PRIMARY KEY,
    user_id    VARCHAR NOT NULL REFERENCES users(id),
    email      VARCHAR NOT NULL,
    password   VARCHAR(128) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL,
    deleted_at TIMESTAMP NULL
)