-- Add migration script here

-- Create table if not exists
CREATE TABLE IF NOT EXISTS servers (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    status VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);