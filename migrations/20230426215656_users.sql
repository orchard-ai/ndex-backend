-- Add migration script here
CREATE TYPE account_type AS ENUM ('credentials', 'google', 'apple');

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
)