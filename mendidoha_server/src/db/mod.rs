pub mod user;
pub mod supplier;
pub mod sessions;
pub mod goods;

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
