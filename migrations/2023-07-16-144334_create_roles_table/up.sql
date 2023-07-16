-- Your SQL goes here
CREATE TABLE roles
(
    id         VARCHAR PRIMARY KEY,
    name   VARCHAR(128) NOT NULL,
    code   VARCHAR(128) NOT NULL,
    description   VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL,
    deleted_at TIMESTAMP NULL
)