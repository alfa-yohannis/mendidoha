use crate::models::user::{NewUser, User};
use crate::schema::{self, users};
use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::result::QueryResult;
use diesel::{debug_query, prelude::*};
use log::info;

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
        username: username,
        password: password,
        // password: &crate::db::hash_password(password),
        first_name,
        middle_name,
        last_name,
        created: current_time,
        updated: current_time,
        created_by,
        updated_by: created_by,
    };
 
    let insert_query = diesel::insert_into(users::table).values(&new_user);

    // Print the SQL query
    let sql_string = debug_query::<diesel::pg::Pg, _>(&insert_query).to_string();
    info!("SQL Query: {:?}", sql_string);

    match diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn) {
        Ok(user) => Ok(user),
        Err(e) => {
            // Log the error
            eprintln!("Database insert error: {:?}", e);
            Err(e)
        }
    }
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

/// Function to get user code by username
pub fn get_user_code_by_username(conn: &mut PgConnection, username: &str) -> Option<String> {
    use schema::users::dsl::*;

    match users
        .filter(username.eq(username))
        .select(code)
        .first::<String>(conn)
    {
        Ok(user_code) => Some(user_code),
        Err(_) => None,
    }
}

/// Function to delete a user by username and password
pub fn remove_user(
    conn: &mut PgConnection,
    username_param: &str,
    password_param: &str,
) -> QueryResult<usize> {
    use schema::users::dsl::*;

    diesel::delete(users.filter(username.eq(username_param).and(password.eq(password_param))))
        .execute(conn)
}

/// Function to get a user by username
pub fn get_user_by_username(conn: &mut PgConnection, username_param: &str) -> QueryResult<User> {
    use schema::users::dsl::*;

    users
        .filter(username.eq(username_param))
        .first::<User>(conn)
}
