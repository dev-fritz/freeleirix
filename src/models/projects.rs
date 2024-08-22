use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};
use crate::DbPool;
use crate::schema::projects::dsl::projects;

#[derive(Debug, Deserialize, Serialize, Selectable, Queryable, PartialEq)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Projects {
    id: i32,
    title: String,
    description: Option<String>,
    subscriber_id: i32,
    freelancer_id: Option<i32>,
    value: Option<i32>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
    removed: Option<bool>
}

#[derive(Debug, Deserialize, Insertable, Serialize)]
#[diesel(table_name = crate::schema::projects)]
pub struct CreateProject {
    title: String,
    description: Option<String>,
    subscriber_id: i32,
    freelancer_id: Option<i32>,
    value: Option<i32>,
    created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize, AsChangeset)]
#[diesel(table_name = crate::schema::projects)]
pub struct UpdateProject {
    title: Option<String>,
    description: Option<String>,
    subscriber_id: Option<i32>,
    freelancer_id: Option<i32>,
    value: Option<i32>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
    removed: Option<bool>
}

impl Projects {
    pub fn create_project(pool: &DbPool, new_project: CreateProject) -> Result<Projects, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        diesel::insert_into(projects)
            .values(&new_project)
            .get_result(&mut conn)
    }

    pub fn update_project(pool: &DbPool, project_id: i32, updated_project: UpdateProject) -> Result<Projects, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        diesel::update(projects.find(project_id))
            .set(&updated_project)
            .get_result(&mut conn)
    }

    pub fn delete_project(pool: &DbPool, project_id: i32) -> Result<usize, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        diesel::delete(projects.find(project_id))
            .execute(&mut conn)
    }

    pub fn list_projects(pool: &DbPool) -> Result<Vec<Projects>, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        projects.load::<Projects>(&mut conn)
    }
}