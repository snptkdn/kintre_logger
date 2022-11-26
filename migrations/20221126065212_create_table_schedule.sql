-- Add migration script here
CREATE TABLE IF NOT EXISTS schedule(
    weekday VARCHAR(255),
    event varchar(255)
);