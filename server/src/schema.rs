table! {
    pre_users (id) {
        id -> Int4,
        email -> Varchar,
        token -> Uuid,
        enabled -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    reservations (id) {
        id -> Int4,
        user_id -> Int4,
        start_time -> Timestamp,
        end_time -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        salt -> Varchar,
        enabled -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(reservations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    pre_users,
    reservations,
    users,
);
