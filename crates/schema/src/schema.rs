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

allow_tables_to_appear_in_same_query!(local_users, users,);
