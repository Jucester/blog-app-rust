use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String,
}

use super::schema::posts;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub slug: &'a str,
    pub body: &'a str,
}


impl Post {
    pub fn define_slug(title: &String) -> String {
        title.replace(" ", "_").to_lowercase()
    }

    pub fn create<'a>(conn: &mut PgConnection, post: &NewPostHandler) -> Result<Post, diesel::result::Error> {
        let slug = Post::define_slug(&post.title.clone());

        let new_post = NewPost {
            title: &post.title,
            body: &post.body,
            slug: &slug,
        };

        diesel::insert_into(posts::table).values(new_post).get_result::<Post>(conn)
    }

}
