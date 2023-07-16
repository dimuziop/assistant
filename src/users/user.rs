use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use crate::schema::{users};

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
pub struct NewUserDTO {
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<String>,
}