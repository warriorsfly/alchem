table! {
    local_users (id) {
        id -> Int4,
        user_id -> Int4,
        password_encrypted -> Varchar,
        salt -> Varchar,
        phone -> Nullable<Varchar>,
    }
}

table! {
    room_users (id) {
        id -> Int4,
        room_id -> Int4,
        user_id -> Int4,
        is_admin -> Bool,
    }
}

table! {
    rooms (id) {
        id -> Int4,
        name -> Varchar,
        invite_link -> Varchar,
        owner -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        display_name -> Varchar,
        avatar -> Varchar,
        bio -> Text,
        local -> Bool,
    }
}

joinable!(local_users -> users (user_id));
joinable!(room_users -> rooms (room_id));
joinable!(room_users -> users (user_id));
joinable!(rooms -> users (owner));

allow_tables_to_appear_in_same_query!(
    local_users,
    room_users,
    rooms,
    users,
);
