pub mod user;
pub mod supplier;
pub mod sessions;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn generate_code() -> String {
    use rand::distributions::Uniform;
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    let range = Uniform::from(0..10);
    (0..10).map(|_| rng.sample(&range).to_string()).collect()
}

pub fn hash_password(password: &str) -> String {
    let result = md5::compute(password);
    format!("{:x}", result)
}


// pub mod models;
// pub mod schema;

// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use diesel::result::QueryResult;

// use dotenv::dotenv;
// use md5;
// use rand::distributions::Uniform;
// use rand::{thread_rng, Rng};
// use std::env;

// use chrono::{DateTime, Utc};
// use models::User;
// use schema::users;

// use models::Supplier;
// use schema::suppliers;

// #[derive(Insertable)]
// #[diesel(table_name = users)]
// struct NewUser<'a> {
//     code: &'a str,
//     username: &'a str,
//     password: &'a str, // Store the MD5 hash here
//     first_name: &'a str,
//     middle_name: Option<&'a str>,
//     last_name: &'a str,
//     created: DateTime<Utc>,
//     updated: DateTime<Utc>,
//     created_by: Option<&'a str>,
//     updated_by: Option<&'a str>,
// }

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
// }

// pub fn generate_code() -> String {
//     let mut rng = thread_rng();
//     let range = Uniform::from(0..10);
//     (0..10).map(|_| rng.sample(&range).to_string()).collect()
// }

// pub fn create_user<'a>(
//     conn: &mut PgConnection,
//     username: &'a str,
//     password: &'a str,
//     first_name: &'a str,
//     middle_name: Option<&'a str>,
//     last_name: &'a str,
//     created_by: Option<&'a str>,
// ) -> QueryResult<User> {
//     let random_code = generate_code();
//     let current_time = Utc::now();

//     let new_user = NewUser {
//         code: &random_code,
//         username,
//         password,
//         first_name,
//         middle_name,
//         last_name,
//         created: current_time,
//         updated: current_time,
//         created_by,
//         updated_by: created_by,
//     };

//     diesel::insert_into(users::table)
//         .values(&new_user)
//         .get_result(conn)
// }

// pub fn verify_user_by_code(conn: &mut PgConnection, _username: &str, _reset_code: &str) -> bool {
//     use diesel::prelude::*;
//     use schema::users::dsl::*;

//     let generated_code = "1234"; // Replace with the function generate reset code
//     if _reset_code != generated_code {
//         return false;
//     }

//     match users.filter(username.eq(_username)).first::<User>(conn) {
//         Ok(_) => true,
//         Err(_) => false,
//     }
// }

// pub fn verify_user(conn: &mut PgConnection, _username: &str, _password: &str) -> bool {
//     use schema::users::dsl::*;

//     let hashed_password = hash_password(_password);

//     match users
//         .filter(username.eq(_username))
//         .filter(password.eq(&hashed_password))
//         .first::<User>(conn)
//     {
//         Ok(_) => true,
//         Err(_) => false,
//     }
// }

// pub fn hash_password(password: &str) -> String {
//     let result = md5::compute(password);
//     format!("{:x}", result)
// }

// // Function to update the user's password and timestamp in the database
// pub fn update_user_password(
//     conn: &mut PgConnection,
//     username_param: &str,
//     new_password_param: &str,
// ) -> Result<(), diesel::result::Error> {
//     use schema::users::dsl::*;

//     diesel::update(users.filter(username.eq(username_param)))
//         .set((
//             password.eq(new_password_param),
//             updated.eq(Utc::now().naive_utc()),
//         ))
//         .execute(conn)?;

//     Ok(())
// }

// //--------------


// #[derive(Insertable)]
// #[diesel(table_name = suppliers)]
// struct NewSupplier<'a> {
//     code: &'a str,
//     name: &'a str,
//     created: DateTime<Utc>,
//     updated: DateTime<Utc>,
//     created_by: Option<&'a str>,
//     updated_by: Option<&'a str>,
// }

// pub fn create_supplier<'a>(
//     conn: &mut PgConnection,
//     code: &'a str,
//     name: &'a str,
//     created_by: Option<&'a str>,
// ) -> QueryResult<Supplier> {
//     let random_code = generate_code();
//     let current_time = Utc::now();

//     let new_supplier = NewSupplier {
//         code: &random_code,
//         name: name,
//         created: current_time,
//         updated: current_time,
//         created_by: created_by,
//         updated_by: created_by,
//     };

//     diesel::insert_into(suppliers::table)
//         .values(&new_supplier)
//         .get_result(conn)
// }

// pub fn get_supplier(conn: &mut PgConnection, supplier_id: i32) -> QueryResult<Supplier> {
//     suppliers::table.find(supplier_id).get_result::<Supplier>(conn)
// }

// pub fn update_supplier(
//     conn: &mut PgConnection,
//     supplier_id: i32,
//     code: &str,
//     name: &str,
//     updated_by: Option<&str>,
// ) -> QueryResult<Supplier> {
//     diesel::update(suppliers::table.find(supplier_id))
//         .set((
//             suppliers::code.eq(code),
//             suppliers::name.eq(name),
//             suppliers::updated.eq(Utc::now().naive_utc()),
//             suppliers::updated_by.eq(updated_by),
//         ))
//         .get_result(conn)
// }

// pub fn delete_supplier(conn: &mut PgConnection, supplier_id: i32) -> QueryResult<usize> {
//     diesel::delete(suppliers::table.find(supplier_id)).execute(conn)
// }

// pub fn list_suppliers(conn: &mut PgConnection) -> QueryResult<Vec<Supplier>> {
//     suppliers::table.load::<Supplier>(conn)
// }
