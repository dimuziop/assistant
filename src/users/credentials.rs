use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use crate::schema::{credentials};

#[derive(Debug, Serialize, Queryable, Identifiable, Insertable, Selectable, Clone)]
#[diesel(table_name = credentials)]
pub struct Credentials {
    pub id: String,
    pub user_id: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogIn {
    pub email: String,
    pub password: String,
}