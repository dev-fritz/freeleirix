use chrono::{NaiveDateTime};
use diesel::{AsChangeset, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};
use crate::DbPool;
use crate::schema::users::dsl::users;

#[derive(Debug, Deserialize, Serialize, Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    id: i32,
    name: String,
    email: String,
    description: Option<String>,
    contractor: bool,
    freelancer: bool,
    doc: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
    removed: Option<bool>
}

#[derive(Debug, Deserialize, Insertable, Serialize)]
#[diesel(table_name = crate::schema::users)]
pub struct CreateUser {
    name: String,
    email: String,
    description: Option<String>,
    contractor: bool,
    freelancer: bool,
    doc: String,
    created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Insertable, Serialize, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
    description: Option<String>,
    contractor: Option<bool>,
    freelancer: Option<bool>,
    doc: Option<String>,
    updated_at: Option<NaiveDateTime>,
    removed: Option<bool>
}

impl Users {
    pub fn create_user(pool: &DbPool, user: CreateUser) -> Result<Users, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        diesel::insert_into(users)
            .values(&user)
            .get_result(&mut conn)
    }

    pub fn update_user(pool: &DbPool, user_id: i32, updated_user: UpdateUser) -> Result<Users, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        diesel::update(users.find(user_id))
            .set(&updated_user)
            .get_result(&mut conn)
    }

    pub fn delete_user(pool: &DbPool, user_id: i32) -> Result<usize, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        diesel::delete(users.find(user_id))
            .execute(&mut conn)
    }

    pub fn list_users(pool: &DbPool) -> Result<Vec<Users>, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        users.load::<Users>(&mut conn)
    }
}