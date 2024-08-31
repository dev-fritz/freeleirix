mod config;
mod models;
mod routes;
mod schema;

use crate::config::database::establish_connection;
use crate::routes::{projects, users};
use actix_web::{web, App, HttpServer};
use diesel::{r2d2, PgConnection};
use std::io;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Starting server...");
    let pool = establish_connection();
    println!("Database connection established.");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(users::init_routes)
            .configure(projects::init_routes)
    })
    .workers(4)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
