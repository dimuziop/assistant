#[macro_use]
extern crate rocket;

mod schema;

use diesel::{ExpressionMethods, Identifiable, Insertable, Queryable, QueryDsl, RunQueryDsl, Selectable};
use crate::authorisation::basic_auth::BasicAuth;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::{json, Json, Value};
use rocket::serde::ser::SerializeStruct;
use rocket::serde::Serializer;
use rocket::{Request, State};
use rocket::http::Status;
use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};
use uuid::{Uuid};
use schema::{users, tasks};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;

pub mod authorisation;

#[derive(Debug, Serialize, Deserialize, Clone)]
enum TimeUnits {
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

#[database("postgres")]
struct DbConn(diesel::PgConnection);

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TimeAmount {
    value: i32,
    unit: TimeUnits,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Insertable, Selectable)]
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

#[derive(Debug, Serialize, Deserialize)]
struct NewTaskDTO {
    pub title: String,
    pub description: Option<String>,
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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorisation_header(auth_header) {
                return Outcome::Success(auth);
            }
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[get("/tasks")]
async fn get_all_tasks(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let tasks_response = tasks::table.order(tasks::created_at.desc()).limit(1000).load::<Task>(c).expect("DB error"); // tempoaral panic
        json!(tasks_response)
    }).await
}

#[get("/tasks/<id>")]
async fn get_task(id: String, _auth: BasicAuth, db: DbConn) -> Result<Value, Status> {
    db.run(|c| {
        let db_result: Result<Task, _> = tasks::table.find(id).get_result(c);
        build_response_from_db_result(db_result)
    }).await
}

#[post("/tasks", format = "json", data = "<new_task>")]
async fn create_task(_auth: BasicAuth, db: DbConn, new_task: Json<NewTaskDTO>) -> Value {
    let new_task_instance: NewTaskDTO = new_task.into_inner();
    db.run(|c| {
        let result: Task = diesel::insert_into(tasks::table)
            .values(Task {
                title: new_task_instance.title,
                description: new_task_instance.description,
                ..Task::default()
            })
            .get_result(c)
            .expect("DB Error");
        json!(result)
    }).await
}

#[put("/tasks/<id>", format = "json", data = "<new_task>")]
async fn update_task(id: String, _auth: BasicAuth, db: DbConn, new_task: Json<NewTaskDTO>) -> Result<Value, Status> {
    let new_task_instance: NewTaskDTO = new_task.into_inner();
    db.run(|c| {
        let tasks_responses: Result<Task, _> = diesel::update(tasks::table.filter(tasks::id.eq(id)))
            .set((
                tasks::title.eq(new_task_instance.title),
                tasks::description.eq(new_task_instance.description),
            ))
            .get_result(c);

        build_response_from_db_result(tasks_responses)
    }).await
}

pub fn build_response_from_db_result(tasks_responses: Result<Task, Error>) -> Result<Value, Status> {
    match tasks_responses {
        Err(error) => {
            println!("Logging error: {:#?}", error);
            Err(Status::NotFound)
        }
        Ok(task) => { Ok(json!(task)) }
    }
}

#[delete("/tasks/<id>")]
async fn delete_task(id: String, _auth: BasicAuth, db: DbConn) -> Status {
    let deleted = db.run(|c| {
        let result = diesel::delete(tasks::table.filter(tasks::id.eq(id))).execute(c).expect("Error deleting task");
        result
    }).await;
    if deleted == 0 {
        return Status::NotFound;
    }
    Status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!({"Error": "Resource not found"})
}

#[catch(401)]
fn unauthorized() -> Value {
    json!({"Error": "Unauthorized"})
}

#[catch(422)]
fn unprocessable_entity() -> Value {
    json!({
        "Error": "Unauthorized",
        "Code": 422
    })
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_all_tasks,
                get_task,
                create_task,
                update_task,
                delete_task,
            ],
        )
        .attach(DbConn::fairing())
        .register("/", catchers![not_found,unauthorized, unprocessable_entity])
        .launch()
        .await;
}
