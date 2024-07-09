pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::env;

use models::User;
use schema::users;

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser<'a> {
    user_id: &'a str,
    username: &'a str,
    password: &'a str,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn generate_user_id() -> String {
    let mut rng = thread_rng();
    let range = Uniform::from(0..10);
    (0..10).map(|_| rng.sample(&range).to_string()).collect()
}

pub fn create_user<'a>(conn: &mut PgConnection, username: &'a str, password: &'a str) -> User {
    let random_user_id = generate_user_id();

    let new_user = NewUser {
        user_id: &random_user_id,
        username,
        password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn verify_user(conn: &mut PgConnection, _username: &str, _password: &str) -> bool {
    use schema::users::dsl::*;

    match users
        .filter(username.eq(_username))
        .filter(password.eq(_password))
        .first::<User>(conn)
    {
        Ok(_) => true,
        Err(_) => false,
    }
}
