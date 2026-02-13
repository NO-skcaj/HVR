use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use self::models::*;
use self::schema::blogs::dsl::*;
use crate::schema::posts;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_blog_post(conn: &mut SqliteConnection, title: &str, date: &str, preview: &str, body: &str) -> Post {
    let new_post = NewBlog { title, body };

    diesel::insert_into(blogs::table)
        .values(&new_post)
        .returning(Blog::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}