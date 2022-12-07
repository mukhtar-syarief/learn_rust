// @generated automatically by Diesel CLI.

diesel::table! {
    user_detail (id) {
        id -> Int4,
        fullname -> Text,
        nickname -> Varchar,
        address -> Nullable<Text>,
        age -> Int4,
        is_adult -> Nullable<Bool>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    user_detail,
    users,
);
