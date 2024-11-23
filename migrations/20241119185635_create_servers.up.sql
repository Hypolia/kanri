-- Add migration script here

-- Create table if not exists
CREATE TABLE IF NOT EXISTS servers (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    player_count INTEGER NOT NULL DEFAULT 0,
    max_player_count INTEGER NOT NULL,
    server_type VARCHAR(255) NOT NULL,
    status VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    metadata JSONB
);