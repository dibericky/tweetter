CREATE TABLE tweets (
  id VARCHAR PRIMARY KEY,
  author_id VARCHAR NOT NULL,
  message VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL
)
