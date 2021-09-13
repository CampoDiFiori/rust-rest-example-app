table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        user_id -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        created_at -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
