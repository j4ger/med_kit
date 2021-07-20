CREATE TABLE reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    uploader_id INTEGER NOT NULL,
    filename VARCHAR,
    download_url VARCHAR NOT NULL,
    upload_time TIMESTAMP NOT NULL
);
