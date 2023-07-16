use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Identifiable, Insertable, Selectable, Clone)]
pub struct Credentials {
    pub id: String,
    pub user: String,
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