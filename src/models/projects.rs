use crate::schema::projects::{dsl::projects, removed};
use crate::DbPool;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};

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
    removed: Option<bool>,
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
    removed: Option<bool>,
}

impl Projects {
    pub fn create_project(
        pool: &DbPool,
        project: CreateProject,
    ) -> Result<Projects, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        diesel::insert_into(projects)
            .values(&project)
            .get_result(&mut conn)
    }

    pub fn update_project(
        pool: &DbPool,
        project_id: i32,
        updated_project: UpdateProject,
    ) -> Result<Projects, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        diesel::update(projects.find(project_id))
            .set(&updated_project)
            .get_result(&mut conn)
    }

    pub fn delete_project(pool: &DbPool, project_id: i32) -> Result<usize, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        diesel::update(projects.find(project_id))
            .set(removed.eq(true))
            .execute(&mut conn)
            
    }

    pub fn list_projects(pool: &DbPool) -> Result<Vec<Projects>, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        projects.filter(removed.ne(true)).load::<Projects>(&mut conn)
    }
}
