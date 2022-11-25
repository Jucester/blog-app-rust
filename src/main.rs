
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use models::{NewPost, Post};
use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;
use diesel::prelude::*;


use self::schema::posts;
use self::schema::posts::dsl::*;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn connect_db() -> DbPool {

    let db_url = env::var("DATABASE_URL").expect("Db url missing");
    // PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Connection failed to {}", db_url))
    //
    let conn = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(conn).expect("Something went wrong.")
}

fn insert(conn: &mut PgConnection, new_post: NewPost) -> Post {
    diesel::insert_into(posts::table).values(new_post).get_result::<Post>(conn).expect("Insert failed")
}

// #[get("/")]
// async fn hello_world() -> impl Responder {
//     HttpResponse::Ok().body("Hello World")
// }


#[get("/posts")]
async fn get_posts(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Error connecting db.");

    match web::block(move || {posts.load::<Post>(&mut conn)}).await {
        Ok(data) => {
            println!("{:?}", data);

            let res = String::new();

            // for post in data {
            //     res = format!("{}\n{}\n", post.title, post.body);
            // }


            HttpResponse::Ok().body(format!("{:?}", data))
        },
        Err(err) => HttpResponse::Ok().body("Fail")
    }

    // let posts_res = posts.load::<Post>(conn).expect("Error executing query");

    // for post in posts_res {
    //     println!("{:?}", post);
    // }
    //
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let conn = connect_db();
    HttpServer::new(move || {
        App::new().service(get_posts).app_data(web::Data::new(conn.clone()))
    }).bind(("0.0.0.0", 9900)).unwrap().run().await

    // let conn = &mut connect_db();

    // let mut post_title = String::new();
    // let mut post_body = String::new();

    // println!("Insert your new pst data: ");
    // println!("Title: ");
    // std::io::stdin().read_line(&mut post_title).unwrap();

    // println!("Body: ");
    // std::io::stdin().read_line(&mut post_body).unwrap();

    // let splitted: Vec<&str> = post_title.split(" ").collect();
    // let post_slug = splitted.join("_");

    // let new_post = NewPost { title: post_title.trim(), slug: post_slug.trim(), body: post_body.trim() };

    // insert(conn, new_post);
    //
    // let updated_post: Post = diesel::update(posts.filter(id.eq(2))).set(title.eq("Second Updated posts")).get_result(conn).expect("Something wnet wrong in update");


    // let deleted = diesel::delete(posts.filter(title.like("%second%"))).execute(conn).expect("Error deleting");

}
