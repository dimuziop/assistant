use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Serialize};
use uuid::Uuid;
use crate::schema::tasks;

#[derive(Debug, Serialize, Queryable, Identifiable, Insertable, Selectable, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub estimated_time: Option<String>,
    //estimated_time: Option<TimeAmount>,
    /*#[serde(with = "time::serde::rfc3339")]
    initial_times: Vec<Timestamp>,
    #[serde(with = "time::serde::rfc3339")]
    end_times: Vec<Timestamp>,*/
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: "New Task".to_string(),
            description: None,
            estimated_time: None,
            /*initial_times: Vec::default(),
            end_times: Vec::default(),*/
            created_at: NaiveDateTime::default(),
            updated_at: None,
            deleted_at: None,
        }
    }
}