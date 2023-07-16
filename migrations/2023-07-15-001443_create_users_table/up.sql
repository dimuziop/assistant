-- Your SQL goes here
CREATE TABLE users
(
    id         VARCHAR PRIMARY KEY,
    name       VARCHAR   NOT NULL,
    last_name  VARCHAR   NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL,
    deleted_at TIMESTAMP NULL
)
