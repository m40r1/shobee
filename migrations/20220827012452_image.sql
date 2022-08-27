-- Add migration script here
-- https://cf.shopee.com.br/file/
-- img url format
CREATE OR REPLACE TABLE bitem_image(
id BIGINT NOT NULL PRIMARY KEY AUTO_INCREMENT,
main_image char(32),
other_images varchar(320)
);
