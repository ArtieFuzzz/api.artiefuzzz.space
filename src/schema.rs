// @generated automatically by Diesel CLI.

diesel::table! {
    logins (username) {
        username -> Text,
        pwd -> Text,
    }
}
