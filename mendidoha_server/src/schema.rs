// @generated automatically by Diesel CLI.

diesel::table! {
    test_table (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 10]
        user_id -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 32]
        password -> Varchar,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        middle_name -> Nullable<Varchar>,
        #[max_length = 50]
        last_name -> Varchar,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    test_table,
    users,
);
