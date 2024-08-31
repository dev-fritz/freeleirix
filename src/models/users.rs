use crate::schema::users::{dsl::users, removed};
use crate::DbPool;
use crate::config::auth;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    id: i32,
    name: String,
    email: String,
    description: Option<String>,
    contractor: Option<bool>,
    freelancer: Option<bool>,
    doc: Option<String>,
    password: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
    removed: Option<bool>,
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
    password: String,
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
    password: Option<String>,
    updated_at: Option<NaiveDateTime>,
    removed: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserResponse {
    id: i32,
    name: String,
    email: String,
    description: Option<String>,
    contractor: Option<bool>,
    freelancer: Option<bool>,
    doc: Option<String>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
    removed: Option<bool>,
}

impl From<Users> for UserResponse {
    fn from(user: Users) -> Self {
        UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            description: user.description,
            contractor: user.contractor,
            freelancer: user.freelancer,
            doc: user.doc,
            created_at: user.created_at,
            updated_at: user.updated_at,
            removed: user.removed,
        }
    }
}

impl Users {
    pub fn create_user(pool: &DbPool, user: CreateUser) -> Result<UserResponse, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");

        let hashed_password = auth::encode_password(user.password.as_str());
        let user_with_hashed_password = CreateUser {
            password: hashed_password,
            ..user
        };

        let new_user: Users = diesel::insert_into(users)
            .values(&user_with_hashed_password)
            .get_result(&mut conn)?;

        Ok(new_user.into())
    }

    pub fn update_user(
        pool: &DbPool,
        user_id: i32,
        mut updated_user: UpdateUser,
    ) -> Result<UserResponse, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");

        if let Some(ref password) = updated_user.password {
            updated_user.password = Some(auth::encode_password(password.as_str()));
        }

        let updated: Users = diesel::update(users.find(user_id))
            .set(&updated_user)
            .get_result(&mut conn)?;

        Ok(updated.into())  // Conversão automática via From trait
    }

    pub fn delete_user(pool: &DbPool, user_id: i32) -> Result<usize, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        diesel::update(users.find(user_id))
            .set(removed.eq(true))
            .execute(&mut conn)
    }

    pub fn list_users(pool: &DbPool) -> Result<Vec<UserResponse>, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");

        let user_list = users
            .filter(removed.ne(true))
            .load::<Users>(&mut conn)?;

        Ok(user_list.into_iter().map(UserResponse::from).collect())
    }

    pub fn login(pool: &DbPool, login_data: auth::Login) -> Result<String, diesel::result::Error> {
        use crate::schema::users::dsl::{email, users};

        let mut conn = pool.get().expect("Failed to get DB connection from pool");

        let user = users
            .filter(email.eq(login_data.email))
            .first::<Users>(&mut conn)?;

        let user_data = auth::verify_password(login_data.password.as_str(), user.password.as_str());
        
        if user_data {
            Ok(auth::generate_token(user.id, &user.email))
        } else {
            Err(diesel::result::Error::NotFound)
        }
    }
}