use crate::models::projects::{CreateProject, Projects, UpdateProject};
use crate::DbPool;
use actix_web::{web, HttpResponse, Responder};

pub async fn list_projects(pool: web::Data<DbPool>) -> impl Responder {
    let result = Projects::list_projects(&pool);

    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_project(
    pool: web::Data<DbPool>,
    new_project: web::Json<CreateProject>,
) -> impl Responder {
    let new_project = new_project.into_inner();
    let result = Projects::create_project(&pool, new_project);

    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_project(
    pool: web::Data<DbPool>,
    project_id: web::Path<i32>,
    updated_project: web::Json<UpdateProject>,
) -> impl Responder {
    let updated_project = updated_project.into_inner();
    let result = Projects::update_project(&pool, project_id.into_inner(), updated_project);

    match result {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_project(pool: web::Data<DbPool>, project_id: web::Path<i32>) -> impl Responder {
    let result = Projects::delete_project(&pool, project_id.into_inner());

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/projects")
            .route(web::get().to(list_projects))
            .route(web::post().to(create_project)),
    )
    .service(
        web::resource("/projects/{id}")
            .route(web::put().to(update_project))
            .route(web::delete().to(delete_project)),
    );
}
