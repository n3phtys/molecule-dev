use diesel;

pub mod models;
pub mod page;
pub mod portlet;
pub mod post;
pub mod schema;
pub mod site;
pub mod structure;
pub mod template;
pub mod user;
pub mod vocabulary;

use database::models::*;
use diesel::prelude::*;
use dotenv::dotenv;
use std;
use std::env;

use schema::users::dsl::*;

#[derive(Debug, Copy, Clone)]
pub struct SiteId(pub i32);

#[derive(Debug, Copy, Clone)]
pub struct UserId(pub i32);

pub enum MoleculeEntity {
    Structure,
    Post,
    Template,
    File,
    User,
    Vocabulary,
    VocabularyEntry,
    Site,
    Page,
    Portlet,
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

embed_migrations!();

#[cfg(test)]
pub fn establish_in_memory() -> SqliteConnection {
    let database_url = ":memory:";
    let conn = SqliteConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url));

    embedded_migrations::run_with_output(&conn, &mut std::io::stdout());

    return conn;
}
