use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::QueryResult;
use chrono::Utc;
use crate::models::user::{User, NewUser};
use crate::schema::{self, users};

pub fn create_user<'a>(
    conn: &mut PgConnection,
    username: &'a str,
    password: &'a str,
    first_name: &'a str,
    middle_name: Option<&'a str>,
    last_name: &'a str,
    created_by: Option<&'a str>,
) -> QueryResult<User> {
    let random_code = crate::db::generate_code();
    let current_time = Utc::now();

    let new_user = NewUser {
        code: &random_code,
        username,
        password: &crate::db::hash_password(password),
        first_name,
        middle_name,
        last_name,
        created: current_time,
        updated: current_time,
        created_by,
        updated_by: created_by,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn verify_user_by_code(conn: &mut PgConnection, _username: &str, _reset_code: &str) -> bool {
    use schema::users::dsl::*;

    let generated_code = "1234"; // Replace with the function generate reset code
    if _reset_code != generated_code {
        return false;
    }

    match users.filter(username.eq(_username)).first::<User>(conn) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn verify_user(conn: &mut PgConnection, _username: &str, _password: &str) -> bool {
    use schema::users::dsl::*;

    let hashed_password = crate::db::hash_password(_password);

    match users
        .filter(username.eq(_username))
        .filter(password.eq(&hashed_password))
        .first::<User>(conn)
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

// Function to update the user's password and timestamp in the database
pub fn update_user_password(
    conn: &mut PgConnection,
    username_param: &str,
    new_password_param: &str,
) -> Result<(), diesel::result::Error> {
    use schema::users::dsl::*;

    diesel::update(users.filter(username.eq(username_param)))
        .set((
            password.eq(new_password_param),
            updated.eq(Utc::now().naive_utc()),
        ))
        .execute(conn)?;

    Ok(())
}
