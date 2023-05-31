-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; -- Enable UUID extension

CREATE TABLE members (
    member_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    borrowed INT NOT NULL DEFAULT 0,
    privilege BOOLEAN NOT NULL
);