-- Add migration script here
CREATE TYPE account_type AS ENUM ('credentials', 'google', 'apple');

CREATE TYPE integration_platform as ENUM ('file', 'notion', 'google', 'discord', 'slack');

CREATE TABLE userdb.users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255), -- used only for password-based authentication
    oauth_provider_id VARCHAR(255), -- used only for OAuth-based authentication
    oauth_access_token VARCHAR(2000), -- used only for OAuth-based authentication
    date_of_birth DATE,
    phone_number VARCHAR(20),
    city VARCHAR(100),
    country VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    account_type account_type NOT NULL
);

CREATE TABLE userdb.integrations (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    oauth_provider_id VARCHAR(255),
    access_token VARCHAR(2000),
    email VARCHAR(255) NOT NULL,
    extra JSONB,
    scopes VARCHAR(255)[],
    platform integration_platform NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES userdb.users (id) ON DELETE CASCADE,
    UNIQUE (user_id, email, platform)
);

CREATE TABLE userdb.typesense (
    user_id INTEGER PRIMARY KEY,
    api_id INTEGER NOT NULL,
    api_key VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES userdb.users (id) ON DELETE CASCADE
);