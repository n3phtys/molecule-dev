#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;

pub mod schema;
pub mod models;
pub mod microservice;
pub mod common;
pub mod database;
pub mod datasourcing;
pub mod config;
pub mod messaging;
pub mod cache;
pub mod authentication;
pub mod authorization;
pub mod templating;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::NewPost;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &SqliteConnection, title: &str, body: &str) -> usize {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}