mod config;
mod schema;
mod models;
mod routes;

use std::io;
use actix_web::{web, App, HttpServer};
use diesel::{r2d2, PgConnection};
use crate::routes::{users, projects};
use crate::config::database::establish_connection;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;


#[actix_web::main]
async fn main() -> io::Result<()> {
    let pool = establish_connection();
    HttpServer::new(move ||
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(users::init_routes)
            .configure(projects::init_routes)
    )
    .workers(4)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}