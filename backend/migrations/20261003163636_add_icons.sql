-- Add migration script here
ALTER TABLE sections   ADD COLUMN icon TEXT;
ALTER TABLE categories ADD COLUMN icon TEXT;