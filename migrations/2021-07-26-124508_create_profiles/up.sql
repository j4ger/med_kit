CREATE TABLE profiles (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  submit_time TIMESTAMP NOT NULL,

  name  VARCHAR NOT NULL,
  id_card_number VARCHAR NOT NULL,
  birth_date TIMESTAMP NOT NULL,
  profession  VARCHAR NOT NULL,
  address VARCHAR NOT NULL,
  phone VARCHAR NOT NULL,
  sample_time TIMESTAMP
);

ALTER TABLE products
ADD CONSTRAINT match_profile_id
FOREIGN KEY (profile_id)
REFERENCES profiles (id);

ALTER TABLE profiles
ADD CONSTRAINT match_user_id
FOREIGN KEY (user_id)
REFERENCES users (id);