
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use models::{NewPost, Post};
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;


use self::schema::posts;
use self::schema::posts::dsl::*;

fn connect_db() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Db url missing");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Connection failed to {}", db_url))
}

fn insert(conn: &mut PgConnection, new_post: NewPost) -> Post {
    diesel::insert_into(posts::table).values(new_post).get_result::<Post>(conn).expect("Insert failed")
}

fn main() {
    let conn = &mut connect_db();

    let mut post_title = String::new();
    let mut post_body = String::new();

    println!("Inser your new post data: ");
    println!("Title: ");
    std::io::stdin().read_line(&mut post_title).unwrap();

    println!("Body: ");
    std::io::stdin().read_line(&mut post_body).unwrap();

    let splitted: Vec<&str> = post_title.split(" ").collect();
    let post_slug = splitted.join("_");



    let new_post = NewPost { title: post_title.trim(), slug: post_slug.trim(), body: post_body.trim() };


    insert(conn, new_post);


    let posts_res = posts.load::<Post>(conn).expect("Error executing query");

    for post in posts_res {
        println!("{}", post.title);
    }
}
