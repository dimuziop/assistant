use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Serialize};
use crate::schema::{users_roles, roles};

#[derive(Debug, Serialize, Queryable, Identifiable, Insertable, Selectable, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub code: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Insertable, Selectable, Clone)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name = users_roles)]
pub struct UserRole {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = users_roles)]
pub struct NewUserRole {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
}