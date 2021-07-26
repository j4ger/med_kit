DROP TABLE reports;

ALTER TABLE reports
DROP CONSTRAINT match_uploader_id;

ALTER TABLE products
DROP CONSTRAINT match_report_id;