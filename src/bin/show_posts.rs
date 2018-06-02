extern crate diesel;
extern crate molecule;

use molecule::*;

use molecule::database::models::*;


use diesel::prelude::*;

fn main() {
    use self::molecule::database::schema::posts::dsl::*;

    let connection = database::establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}