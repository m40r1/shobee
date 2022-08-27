-- Add migration script here
CREATE OR REPLACE TABLE bitem_rating(
id BIGINT Primary Key NOT NULL AUTO_INCREMENT,
item_rating BIGINT,
total_count INT,
one_star INT,
two_star INT,
three_star INT,
four_star INT,
five_star INT,
count_context INT,
count_item_image INT
);
