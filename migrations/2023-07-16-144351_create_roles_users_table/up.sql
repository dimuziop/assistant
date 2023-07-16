-- Your SQL goes here
CREATE TABLE users_roles
(
    id         VARCHAR PRIMARY KEY,
    user_id  VARCHAR   NOT NULL REFERENCES users(id),
    role_id  VARCHAR   NOT NULL REFERENCES roles(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NULL,
    deleted_at TIMESTAMP NULL
)