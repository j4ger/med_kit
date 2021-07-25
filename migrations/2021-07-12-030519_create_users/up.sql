CREATE TYPE ROLE AS ENUM ('User','Staff','Admin');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR,
    wechat_id VARCHAR UNIQUE,
    user_role ROLE NOT NULL DEFAULT 'User',
    password_hashed VARCHAR,
    phone_number INTEGER,
    sign_up_time TIMESTAMP NOT NULL 
);

ALTER TABLE profiles
ADD CONSTRAINT match_user_id
FOREIGN KEY (user_id)
REFERENCES users (id);