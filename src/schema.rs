// @generated automatically by Diesel CLI.

diesel::table! {
    seaql_migrations (version) {
        version -> Varchar,
        applied_at -> Int8,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        name -> Text,
        password -> Text,
        email -> Text,
        dob -> Nullable<Date>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    seaql_migrations,
    user,
);
