-- Your SQL goes here
CREATE TABLE motds (
  id SERIAL PRIMARY KEY,
  description VARCHAR NOT NULL,
  game_mode VARCHAR NOT NULL,
  max_players INTEGER,
  name VARCHAR NOT NULL,
  ret_msg VARCHAR,
  start_date_time VARCHAR NOT NULL,
  team_1_gods_csv VARCHAR,
  team_2_gods_csv VARCHAR,
  mode TEXT NOT NULL
)
