CREATE TYPE ROLE AS ENUM ('User','Staff','Admin');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    uuid UUID UNIQUE NOT NULL,
    wechat_id VARCHAR UNIQUE,
    user_role ROLE NOT NULL DEFAULT 'User',
    password_hashed VARCHAR,
    phone_number INTEGER,
    sign_up_time TIMESTAMP NOT NULL 
);