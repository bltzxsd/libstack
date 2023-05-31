-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; -- Enable UUID extension

CREATE TABLE books (
    book_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    publication_year INT NOT NULL,
    isbn TEXT,
    availability_status BOOLEAN NOT NULL
);