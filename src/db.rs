use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

// #[derive(Debug, Serialize)]
// pub struct Response<'r> {
//     status: &'r str,
//     msg: &'r str,
// }

// impl<'r> Response<'r> {
//     pub fn new(status: &'r str, msg: &'r str) -> Self {
//         Self { status, msg }
//     }
// }

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
