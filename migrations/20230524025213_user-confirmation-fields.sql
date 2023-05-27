-- Add migration script here
ALTER TABLE userdb.users ADD active BOOLEAN NOT NULL DEFAULT FALSE;

CREATE TABLE userdb.confirmation_hash (
    hash_key VARCHAR(255) PRIMARY KEY
    FOREIGN KEY (user_id) REFERENCES userdb.users (id) ON DELETE CASCADE,
    -- hash_key and user_id are not unique together, user might request another link if lost first one
);

