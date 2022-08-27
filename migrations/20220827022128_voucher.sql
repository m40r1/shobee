-- Add migration script her
CREATE OR REPLACE TABLE voucher(
id BIGINT PRIMARY KEY NOT NULL,
code VARCHAR(100),
laber VARCHAR(80)
)
