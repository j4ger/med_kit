CREATE TYPE STAGE AS ENUM ('Initialized','Submitted','Finished');

CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    product_barcode VARCHAR NOT NULL UNIQUE,
    profile_id INTEGER,
    init_time TIMESTAMP NOT NULL,
    current_stage STAGE NOT NULL DEFAULT 'Initialized',
    report_id uuid
);

ALTER TABLE products
ADD CONSTRAINT match_profile_id
FOREIGN KEY (profile_id)
REFERENCES profiles (id);
