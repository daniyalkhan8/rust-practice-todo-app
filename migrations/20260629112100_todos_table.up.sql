-- Add up migration script here
CREATE TABLE todo (
     id SERIAL PRIMARY KEY,
     title VARCHAR(255) NOT NULL,
     done BOOLEAN DEFAULT false,
     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);