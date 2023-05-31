#![warn(clippy::all, clippy::restriction, clippy::pedantic, clippy::nursery, clippy::cargo)]

use actix_web::middleware;
use actix_web::web::Data;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::Responder;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

mod books;
mod db;
mod members;
mod schema;

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    actix_web::HttpResponse::Ok().json("hello from rust!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // set logger and env
    env_logger::init();
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");
    dotenvy::dotenv().ok();
    // set up database config
    let database_url = std::env::var("DATABASE_URL").expect("Database url is not set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");

    // start server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(hello)
            .service(books::handlers::add_book)
            .service(books::handlers::fetch_book)
            .service(books::handlers::change_book)
            .service(books::handlers::remove_book)
            .service(members::handlers::create_member)
            .service(members::handlers::fetch_member)
            .service(members::handlers::change_member)
            .service(members::handlers::remove_member)
    })
        .bind("127.0.0.1:9090")?
        .run().await
}
