use std::env;

use diesel::{pg::PgConnection, Connection};

pub fn open_connection() -> PgConnection {
    dotenv::dotenv().ok();
    PgConnection::establish(
        env::var("DATABASE_URL")
            .expect("DATABASE_URL env variable is not set.")
            .as_str(),
    )
    .expect("Error connecting to database")
}

