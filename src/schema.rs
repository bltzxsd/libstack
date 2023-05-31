// @generated automatically by Diesel CLI.

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
    loans (loan_id) {
        loan_id -> Uuid,
        member_id -> Uuid,
        book_id -> Uuid,
        loan_date -> Date,
        due_date -> Date,
        return_date -> Nullable<Date>,
        fine -> Nullable<Int4>,
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

diesel::allow_tables_to_appear_in_same_query!(books, loans, members,);
