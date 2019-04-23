table! {
    blog (id) {
        id -> Int4,
        data -> Jsonb,
    }
}

table! {
    immortal_user (id) {
        id -> Int4,
        nick_name -> Varchar,
        password -> Varchar,
        role -> Array<Text>,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        sex -> Int4,
        created_at -> Timestamp,
        avatar -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    blog,
    immortal_user,
);
