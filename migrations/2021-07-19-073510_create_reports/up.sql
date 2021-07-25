CREATE TABLE reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    uploader_id INTEGER NOT NULL,
    filename VARCHAR,
    download_url VARCHAR NOT NULL,
    upload_time TIMESTAMP NOT NULL
);

ALTER TABLE reports
ADD CONSTRAINT match_uploader_id
FOREIGN KEY (uploader_id)
REFERENCES users (id);

ALTER TABLE products
ADD CONSTRAINT match_report_id
FOREIGN KEY (report_id)
REFERENCES reports (id);

