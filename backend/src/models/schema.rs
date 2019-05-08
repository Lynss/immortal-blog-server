table! {
    blog (id) {
        id -> Int4,
        data -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    immortal_user (id) {
        id -> Int4,
        nickname -> Varchar,
        password -> Varchar,
        role -> Array<Int4>,
        email -> Varchar,
        phone -> Nullable<Varchar>,
        sex -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        avatar -> Varchar,
    }
}

table! {
    permission (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    role (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    role_permission (id) {
        id -> Int4,
        role_id -> Int4,
        permission_id -> Int4,
        level -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    blog,
    immortal_user,
    permission,
    role,
    role_permission,
);
