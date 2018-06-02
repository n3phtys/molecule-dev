extern crate molecule;
extern crate diesel;

use self::diesel::prelude::*;
use self::molecule::*;
use self::molecule::database::models::Post;
use std::env::args;

fn main() {
    use molecule::database::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = database::establish_connection();

    let _ = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(&connection)
        .expect(&format!("Unable to find post {}", id));

    let post: molecule::database::models::Post = posts
        .find(id)
        .first(&connection)
        .expect(&format!("Unable to find post {}", id));

    println!("Published post {}", post.title);
}