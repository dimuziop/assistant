use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Identifiable, Insertable, Selectable, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}