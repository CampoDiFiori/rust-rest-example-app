#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;

use actix_web::{App, HttpServer, error, web};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("NOT FOUND");
    let database_pool = r2d2::Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_url))
        .expect("Couldn't create database connection pool");

        
    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| error::ErrorBadRequest(err.to_string()));

        let json_config2 = json_config.clone();
        let json_config3 = json_config.clone();

        App::new()
            .data(database_pool.clone())
            .service(routes::get_users)
            .service(routes::get_user)
            .service(routes::create_user)
            .service(routes::update_user)
            .service(routes::delete_user)
            // new endpoints
            .service(routes::get_posts)
            .service(routes::get_post)
            .service( web::resource("/posts").app_data(json_config).route(web::post().to(routes::create_post)))
            .service( web::resource("/update_post").app_data(json_config2).route(web::put().to(routes::update_post)))
            .service(routes::delete_post)
            .service(routes::get_user_with_posts)
            .service(routes::get_users_posts)
            .service( web::resource("/link_post_author").app_data(json_config3).route(web::put().to(routes::link_post_author)))
            .service(routes::unlink_post_author)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
