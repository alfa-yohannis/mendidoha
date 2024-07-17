use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::schema::sessions;
use crate::models::session::{Session, NewSession};
use log::{error, info};

pub fn get_active_session(conn: &mut PgConnection, user_code: &str, device_id: &str) -> Result<Option<Session>, diesel::result::Error> {
    let current_time = Utc::now().to_utc();

    sessions::table
        .filter(sessions::user_code.eq(user_code))
        .filter(sessions::device_id.eq(device_id))
        .filter(sessions::expiry_time.gt(current_time))
        .filter(sessions::start_time.lt(current_time))
        .order_by(sessions::start_time.desc())
        .first(conn)
        .optional()
}

/// Function to create a new session in the database
pub fn create_session(conn: &mut PgConnection, new_session: &NewSession) -> Result<(), diesel::result::Error> {
    match diesel::insert_into(sessions::table).values(new_session).execute(conn) {
        Ok(_) => {
            info!("Session created successfully.");
            Ok(())
        },
        Err(e) => {
            error!("Failed to create session: {:?}", e);
            Err(e)
        },
    }
}

/// Function to retrieve a session by its ID from the database
pub fn get_session_by_id(conn: &mut PgConnection, session_id: i32) -> Result<Option<Session>, diesel::result::Error> {
    match sessions::table.find(session_id).first(conn).optional() {
        Ok(session) => Ok(session),
        Err(e) => {
            error!("Failed to retrieve session by ID {}: {}", session_id, e);
            Err(e)
        },
    }
}

/// Function to update a session in the database
pub fn update_session(conn: &mut PgConnection, session_id: i32, updated_session: &Session) -> Result<(), diesel::result::Error> {
    match diesel::update(sessions::table.find(session_id)).set(updated_session).execute(conn) {
        Ok(_) => {
            info!("Session updated successfully.");
            Ok(())
        },
        Err(e) => {
            error!("Failed to update session ID {}: {}", session_id, e);
            Err(e)
        },
    }
}

/// Function to delete a session from the database
pub fn delete_session(conn: &mut PgConnection, session_id: i32) -> Result<(), diesel::result::Error> {
    match diesel::delete(sessions::table.find(session_id)).execute(conn) {
        Ok(_) => {
            info!("Session deleted successfully.");
            Ok(())
        },
        Err(e) => {
            error!("Failed to delete session ID {}: {}", session_id, e);
            Err(e)
        },
    }
}

/// Example function to list all sessions
pub fn list_sessions(conn: &mut PgConnection) -> Result<Vec<Session>, diesel::result::Error> {
    match sessions::table.load::<Session>(conn) {
        Ok(sessions) => Ok(sessions),
        Err(e) => {
            error!("Failed to list sessions: {}", e);
            Err(e)
        },
    }
}

/// Function to get a session by user_code and device_id
pub fn get_session_by_user_code_and_device_id(
    conn: &mut PgConnection,
    user_code: &str,
    device_id: &str,
) -> Result<Option<Session>, diesel::result::Error> {
    match sessions::table
        .filter(sessions::user_code.eq(user_code))
        .filter(sessions::device_id.eq(device_id))
        .first(conn)
        .optional()
    {
        Ok(session) => Ok(session),
        Err(e) => {
            error!("Failed to get session by user code {} and device ID {}: {}", user_code, device_id, e);
            Err(e)
        },
    }
}
