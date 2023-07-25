// @generated automatically by Diesel CLI.

diesel::table! {
    queue (id) {
        id -> Integer,
        game -> Text,
        path -> Text,
    }
}
