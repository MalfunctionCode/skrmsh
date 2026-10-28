-- Add migration script here
CREATE TYPE hero_tier AS ENUM ('main', 'comfortable', 'learning');

CREATE TABLE player_heroes(
  player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    hero_id SMALLINT NOT NULL REFERENCES heroes(id),
    tier hero_tier NOT NULL,
    PRIMARY KEY (player_id, hero_id)
);