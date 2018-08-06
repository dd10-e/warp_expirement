table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Text,
        hashed_password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
