// @generated automatically by Diesel CLI.

diesel::table! {
    cars (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
    }
}

diesel::table! {
    locations (id) {
        id -> Int4,
        region -> Text,
    }
}

diesel::table! {
    reservations (id) {
        id -> Int4,
        vehicle_type_id -> Int4,
        region_id -> Int4,
        user_id -> Int4,
        pickup_date -> Timestamp,
        return_date -> Timestamp,
    }
}

diesel::table! {
    return_reservation (id) {
        id -> Int4,
        user_id -> Int4,
        region_id -> Int4,
        odometer -> Int4,
        full_tank -> Bool,
        time -> Date,
    }
}

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

diesel::joinable!(reservations -> cars (vehicle_type_id));
diesel::joinable!(reservations -> locations (region_id));
diesel::joinable!(reservations -> users (user_id));
diesel::joinable!(return_reservation -> locations (region_id));
diesel::joinable!(return_reservation -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cars,
    locations,
    reservations,
    return_reservation,
    user_detail,
    users,
);
