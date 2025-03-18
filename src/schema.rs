// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 256]
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
