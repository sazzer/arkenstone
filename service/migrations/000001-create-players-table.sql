CREATE TABLE players(
  player_id UUID PRIMARY KEY,
  version UUID NOT NULL,
  created TIMESTAMP WITH TIME ZONE NOT NULL,
  updated TIMESTAMP WITH TIME ZONE NOT NULL,

  name TEXT NOT NULL,
  avatar_url TEXT NULL,
  logins JSONB NOT NULL
);