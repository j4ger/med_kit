CREATE TABLE profiles (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  submit_time TIMESTAMP NOT NULL
);