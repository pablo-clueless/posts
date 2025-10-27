mod database;
mod handlers;
// mod lib;
mod models;
mod schema;

use actix_web::{App, HttpServer, web};
use deadpool_diesel::postgres::{Manager, Pool};
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager)
        .build()
        .expect("Failed to create pool");

    let database = database::Database::new(pool);

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .service(
                web::scope("/api")
                    .route("/users", web::post().to(handlers::create_user))
                    .route("/users/{id}", web::get().to(handlers::get_user))
                    .route(
                        "/users/username/{username}",
                        web::get().to(handlers::get_user_by_username),
                    )
                    .route(
                        "/users/{user_id}/posts",
                        web::get().to(handlers::get_user_posts),
                    )
                    .route(
                        "/users/{user_id}/followers",
                        web::get().to(handlers::get_user_followers),
                    )
                    .route(
                        "/users/{user_id}/following",
                        web::get().to(handlers::get_user_following),
                    )
                    .route("/posts", web::post().to(handlers::create_post))
                    .route("/posts", web::get().to(handlers::get_posts))
                    .route(
                        "/posts/{post_id}/comments",
                        web::get().to(handlers::get_post_comments),
                    )
                    .route("/comments", web::post().to(handlers::create_comment))
                    .route(
                        "/posts/{post_id}/like/{user_id}",
                        web::post().to(handlers::like_post),
                    )
                    .route(
                        "/posts/{post_id}/share/{user_id}",
                        web::post().to(handlers::share_post),
                    )
                    .route(
                        "/users/{follower_id}/follow/{following_id}",
                        web::post().to(handlers::follow_user),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
