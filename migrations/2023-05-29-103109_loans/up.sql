-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; -- Enable UUID extension

CREATE TABLE loans (
    loan_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    member_id UUID NOT NULL,
    book_id UUID NOT NULL,
    loan_date DATE NOT NULL,
    due_date DATE NOT NULL,
    return_date DATE,
    fine INT DEFAULT 0,
    FOREIGN KEY (member_id) REFERENCES members (member_id),
    FOREIGN KEY (book_id) REFERENCES books (book_id)
);