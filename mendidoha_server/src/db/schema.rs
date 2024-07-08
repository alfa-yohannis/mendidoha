// @generated automatically by Diesel CLI.

// diesel::table! {
//     test_table (id) {
//         id -> Int4,
//         #[max_length = 50]
//         name -> Nullable<Varchar>,
//     }
// }

diesel::table! {
    users (id) {
        id -> Int4,
        user_id -> Varchar,
        username -> Varchar,
        password -> Varchar,
    }
}