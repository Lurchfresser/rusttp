// @generated automatically by Diesel CLI.

diesel::table! {
    overflows (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        body -> Text,
        published -> Date,
        rating -> Int4,
        overflow_id -> Int4,
    }
}
