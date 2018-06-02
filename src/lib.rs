#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate diesel_migrations;

pub mod authentication;
pub mod authorization;
pub mod cache;
pub mod common;
pub mod config;
pub mod database;
pub mod datasourcing;
pub mod messaging;
pub mod microservice;
pub mod templating;

use database::models;
use database::schema;
use diesel::prelude::*;

use self::models::NewPost;

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
