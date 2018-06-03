use diesel;

pub mod models;
pub mod schema;

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

pub fn update_user(conn: &SqliteConnection, user: &User) -> Result<usize, diesel::result::Error> {
    diesel::update(users.find(user.user_id))
        .set(user)
        .execute(conn)
}

pub fn create_user(
    conn: &SqliteConnection,
    site_id: SiteId,
    name: &str,
) -> std::result::Result<User, diesel::result::Error> {
    use database::models::NewUser;
    use schema::users;

    let new_user = NewUser {
        screen_name: name,
        original_site_id: site_id.0,
    };

    return match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
    {
        Ok(n) => users.order(user_id.desc()).first::<User>(conn),
        Err(e) => Err(e),
    };
}

pub fn delete_user(
    conn: &SqliteConnection,
    id: UserId,
) -> std::result::Result<usize, diesel::result::Error> {
    return diesel::delete(users.filter(user_id.eq(id.0))).execute(conn);
}

pub fn retrieve_users(
    conn: &SqliteConnection,
    site_id: SiteId,
    min_including: u64,
    max_excluding: u64,
) -> std::result::Result<std::vec::Vec<User>, diesel::result::Error> {
    let results = users
        .filter(original_site_id.eq(site_id.0))
        .offset(min_including as i64)
        .limit(max_excluding as i64 - min_including as i64)
        .load::<User>(conn);

    return results;
}

#[test]
fn create_two_users() {
    let conn = establish_in_memory();
    let site_id = SiteId(42i32);
    let name = "alice";
    let old = retrieve_users(&conn, site_id, 0, 100).expect("should retrieve users");
    create_user(&conn, site_id, name).expect("should create user");
    create_user(&conn, site_id, name).expect("should create user");
    let new = retrieve_users(&conn, site_id, 0, 100).expect("should retrieve users");
    assert_eq!(old.len() + 2, new.len());
}

#[test]
fn create_one_user_and_delete() {
    let conn = establish_in_memory();
    use diesel::*;
    let site_id = SiteId(42i32);
    let name = "alice";
    let number_of_users = create_user(&conn, site_id, name).expect("should create user");
    let my_users = retrieve_users(&conn, site_id, 0, 100).expect("should retrieve users");
    let del = delete_user(&conn, UserId(my_users[0].user_id)).expect("delete user should work");
    assert_eq!(del, 1);
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
