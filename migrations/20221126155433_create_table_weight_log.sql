-- Add migration script here
CREATE TABLE IF NOT EXISTS weight_log(
    weight decimal,
    date  date
);