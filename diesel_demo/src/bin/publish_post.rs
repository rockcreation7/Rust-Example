extern crate diesel_demo;
extern crate diesel;

use self::diesel_demo::*;

use self::diesel::prelude::*;

use diesel_demo::models::Post;
use std::env::args;

fn main() {
    use diesel_demo::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let post: Post = posts
        .find(id)
        .first(&connection)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));

    diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(&connection)
        .unwrap();

    println!("Published post {}", post.title);
}