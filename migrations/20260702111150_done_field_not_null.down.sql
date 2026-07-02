-- Add down migration script here
ALTER TABLE todo ALTER COLUMN done DROP NOT NULL;