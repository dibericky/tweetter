CREATE TABLE user_profile (
  id VARCHAR PRIMARY KEY,
  nickname VARCHAR NOT NULL,
  num_tweets INTEGER NOT NULL,
  following INTEGER NOT NULL,
  follower INTEGER NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL
)
