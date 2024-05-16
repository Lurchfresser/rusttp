extern crate diesel;

pub mod overlow_models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use overlow_models::Overflow;
use std::env;


use crate::overlow_models::NewOverflow;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_overflow_now<'a>(conn: &mut PgConnection, title:&'a str, body: &'a str, overflow_id: i32,author: &'a str, rating: i32) -> Overflow {
    use crate::schema::overflows;

    let new_overflow = NewOverflow::from_now(title, author, body, overflow_id, rating);

    diesel::insert_into(overflows::table)
        .values(&new_overflow)
        .returning(Overflow::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}