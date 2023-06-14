// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "loan_status"))]
    pub struct LoanStatus;
}

diesel::table! {
    books (book_id) {
        book_id -> Uuid,
        title -> Text,
        author -> Text,
        publication_year -> Int4,
        isbn -> Nullable<Text>,
        availability_status -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::LoanStatus;

    loans (loan_id) {
        loan_id -> Uuid,
        member_id -> Uuid,
        book_id -> Uuid,
        loan_date -> Date,
        due_date -> Date,
        return_date -> Nullable<Date>,
        status -> LoanStatus,
    }
}

diesel::table! {
    members (member_id) {
        member_id -> Uuid,
        name -> Text,
        email -> Text,
        borrowed -> Int4,
        privilege -> Bool,
    }
}

diesel::joinable!(loans -> books (book_id));
diesel::joinable!(loans -> members (member_id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    loans,
    members,
);
