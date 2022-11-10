CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  payload VARCHAR NOT NULL,
  event_type VARCHAR NOT NULL,
  aggregate_id VARCHAR NOT NULL,
  created_at TIMESTAMPTZ DEFAULT Now() 
)