-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE players (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    password_hash TEXT NOT NULL,
    username      VARCHAR(50) NOT NULL UNIQUE,
    team_id       UUID REFERENCES teams(id),
    is_team_owner boolean not null default FALSE,
    twitch_link   TEXT DEFAULT NULL,
    discord       TEXT DEFAULT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);