CREATE TABLE profiles (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  submit_time TIMESTAMP NOT NULL,

  name  VARCHAR NOT NULL,
  age   INTEGER NOT NULL,
  sample_time   TIMESTAMP NOT NULL
);

ALTER TABLE products
ADD CONSTRAINT match_profile_id
FOREIGN KEY (profile_id)
REFERENCES profiles (id);

ALTER TABLE profiles
ADD CONSTRAINT match_user_id
FOREIGN KEY (user_id)
REFERENCES users (id);