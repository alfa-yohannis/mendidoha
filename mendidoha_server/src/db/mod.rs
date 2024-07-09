pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use md5;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::env;

use models::User;
use schema::users;

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser<'a> {
    user_id: &'a str,
    username: &'a str,
    password: &'a str, // Store the MD5 hash here
    first_name: &'a str,
    middle_name: Option<&'a str>,
    last_name: &'a str
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn generate_user_id() -> String {
    let mut rng = thread_rng();
    let range = Uniform::from(0..10);
    (0..10).map(|_| rng.sample(&range).to_string()).collect()
}

// pub fn create_user<'a>(conn: &mut PgConnection, username: &'a str, password: &'a str) -> User {
//     let random_user_id = generate_user_id();
//     let hashed_password = hash_password(password);

//     let new_user = NewUser {
//         user_id: &random_user_id,
//         username,
//         password: &hashed_password,
//     };

//     diesel::insert_into(users::table)
//         .values(&new_user)
//         .get_result(conn)
//         .expect("Error saving new user")
// }

pub fn create_user<'a>(
    conn: &mut PgConnection,
    username: &'a str,
    password: &'a str,
    first_name: &'a str,
    middle_name: Option<&'a str>,
    last_name: &'a str,
) -> Result<User, diesel::result::Error> {
    let random_user_id = generate_user_id();

    let new_user = NewUser {
        user_id: &random_user_id,
        username,
        password,
        first_name,
        middle_name,
        last_name,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn verify_user(conn: &mut PgConnection, _username: &str, _password: &str) -> bool {
    use schema::users::dsl::*;

    let hashed_password = hash_password(_password);

    match users
        .filter(username.eq(_username))
        .filter(password.eq(&hashed_password))
        .first::<User>(conn)
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn hash_password(password: &str) -> String {
    let result = md5::compute(password);
    format!("{:x}", result)
}
