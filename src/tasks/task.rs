use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use rocket::serde::Deserialize;
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TimeUnits {
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Month,
    Year,
}

impl TimeUnits {
    fn value(&self) -> i64 {
        match *self {
            TimeUnits::Millisecond => 1,
            TimeUnits::Second => 1000,
            TimeUnits::Minute => 60000,
            TimeUnits::Hour => 360000,
            TimeUnits::Day => 8640000,
            TimeUnits::Month => 2628000000,
            TimeUnits::Year => 31536000000,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeAmount {
    value: i32,
    unit: TimeUnits,
}