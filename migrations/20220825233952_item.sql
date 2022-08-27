-- Add migration script here
CREATE OR REPLACE TABLE bitem(
itemid BIGINT PRIMARY KEY NOT NULL,
shopid BIGINT NOT NULL,
#TODO images
stock INT,
sold INT,
hist_solf BIGINT,
likedc INT,
categoryid BIGINT NOT NULL,
brand VARCHAR(100),
price BIGINT NOT NULL,
price_min BIGINT NOT NULL,
price_max BIGINT NOT NULL,
price_min_disc BIGINT,
price_max_disc BIGINT,
price_bfr_disc BIGINT,
discount SMALLINT
);
