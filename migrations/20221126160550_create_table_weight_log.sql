-- Add migration script here
CREATE TABLE IF NOT EXISTS weight_log(
    weight decimal(4,2),
    date  date
);