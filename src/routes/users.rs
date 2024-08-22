use actix_web::{web, HttpResponse, Responder};
use crate::models::users::{Users, CreateUser, UpdateUser};
use crate::DbPool;

pub async fn list_users(pool: web::Data<DbPool>) -> impl Responder {
    let result = Users::list_users(&pool);

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_user(pool: web::Data<DbPool>, new_user: web::Json<CreateUser>) -> impl Responder {
    let new_user = new_user.into_inner();
    let result = Users::create_user(&pool, new_user);

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_user(pool: web::Data<DbPool>, user_id: web::Path<i32>, updated_user: web::Json<UpdateUser>) -> impl Responder {
    let updated_user = updated_user.into_inner();
    let result = Users::update_user(&pool, user_id.into_inner(), updated_user);

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> impl Responder {
    let result = Users::delete_user(&pool, user_id.into_inner());

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(list_users))
            .route(web::post().to(create_user))
    )
    .service(
        web::resource("/users/{id}")
            .route(web::put().to(update_user))
            .route(web::delete().to(delete_user))
    );
}