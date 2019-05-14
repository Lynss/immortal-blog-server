table! {
    blogs (id) {
        id -> Int4,
        data -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    immortal_users (id) {
        id -> Int4,
        nickname -> Varchar,
        password -> Varchar,
        roles -> Array<Int4>,
        email -> Varchar,
        phone -> Nullable<Varchar>,
        sex -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        avatar -> Varchar,
    }
}

table! {
    permissions (id) {
        id -> Int4,
        name -> Varchar,
        status -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    role_permissions (id) {
        id -> Int4,
        role_id -> Int4,
        permission_id -> Int4,
        level -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        status -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    blogs,
    immortal_users,
    permissions,
    role_permissions,
    roles,
);