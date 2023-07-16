-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tasks
(
    id         VARCHAR PRIMARY KEY,
    title      VARCHAR   NOT NULL,
    description   TEXT,
    estimated_time  VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL,
    deleted_at TIMESTAMP NULL
)