extern crate diesel;
extern crate molecule;

use self::diesel::prelude::*;
use self::molecule::database::*;
use std::env::args;

fn main() {
    use molecule::database::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = molecule::database::establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
