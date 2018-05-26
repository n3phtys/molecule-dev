extern crate diesel;
extern crate molecule;

use molecule::*;

use molecule::models::*;


use diesel::prelude::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = establish_connection();


    let new_user = NewUser {
        screen_name: "Just another user",
        original_site_id: 1,
    };

    diesel::insert_into(self::schema::users::table)
        .values(&new_user)
        .execute(&connection)
        .expect("Error saving new user");


    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}: {}", user.user_id, user.screen_name);
    }
}