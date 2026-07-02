-- Add up migration script here
ALTER TABLE todo ALTER COLUMN done SET NOT NULL;