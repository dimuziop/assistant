#[macro_use]
extern crate rocket;

mod schema;
mod tasks;

use diesel::{ExpressionMethods, Identifiable, Insertable, Queryable, QueryDsl, RunQueryDsl, Selectable};
use crate::authorisation::basic_auth::BasicAuth;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::{json, Json, Value};
use rocket::serde::ser::SerializeStruct;
use rocket::serde::Serializer;
use rocket::{Build, Request, Rocket};
use rocket::http::Status;
use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::result::Error;
use diesel_migrations::EmbeddedMigrations;
use crate::tasks::tasks_repository::TasksRepository;
use crate::tasks::task::Task;
use log::{info, warn, error};
use rocket::fairing::AdHoc;
use rocket::futures::TryFutureExt;

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


#[derive(Debug, Serialize, Deserialize)]
struct NewTaskDTO {
    pub title: String,
    pub description: Option<String>,
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorisation_header(auth_header) {
                info!("Successful Auth user: {}", auth.username);
                return Outcome::Success(auth);
            }
        }
        info!("Unauthorized Request");
        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[get("/tasks")]
async fn get_all_tasks(_auth: BasicAuth, db: DbConn) -> Result<Value, Status> {
    db.run(|c| {
        let db_result = TasksRepository::all(c, 1000);
        build_response_from_db_result(db_result)
    }).await
}

#[get("/tasks/<id>")]
async fn get_task(id: String, _auth: BasicAuth, db: DbConn) -> Result<Value, Status> {
    db.run(|c| {
        let db_result: Result<Task, _> = TasksRepository::find(c, id);
        build_response_from_db_result(db_result)
    }).await
}

#[post("/tasks", format = "json", data = "<new_task>")]
async fn create_task(_auth: BasicAuth, db: DbConn, new_task: Json<NewTaskDTO>) -> Result<Value, Status> {
    let new_task_instance: NewTaskDTO = new_task.into_inner();
    db.run(|c| {
        let db_result: Result<Task, _> = TasksRepository::add(c, Task {
            title: new_task_instance.title,
            description: new_task_instance.description,
            ..Task::default()
        });
        build_response_from_db_result(db_result)
    }).await
}

#[put("/tasks/<id>", format = "json", data = "<new_task>")]
async fn update_task(id: String, _auth: BasicAuth, db: DbConn, new_task: Json<NewTaskDTO>) -> Result<Value, Status> {
    let new_task_instance: NewTaskDTO = new_task.into_inner();
    db.run(|c| {
        let db_result: Result<Task, _> = TasksRepository::replace(c, id, Task {
            title: new_task_instance.title,
            description: new_task_instance.description,
            ..Task::default()
        });

        build_response_from_db_result(db_result)
    }).await
}

#[delete("/tasks/<id>")]
async fn delete_task(id: String, _auth: BasicAuth, db: DbConn) -> Result<Value, Status> {
    let deleted = db.run(|c| {
        let result = TasksRepository::soft_delete(c, id);
        result
    }).await;

    deleted
        .map(|task| json!(task))
        .map_err(|err| {
            match err {
                Error::NotFound => {
                    warn!("Logging error: {:#?}", err);
                    Status::NotFound
                }
                _ => {
                    Status::InternalServerError
                }
            }
        })
}

pub fn build_response_from_db_result<T: Serialize>(tasks_responses: Result<T, Error>) -> Result<Value, Status> {
    match tasks_responses {
        Err(error) => {
            return match error {
                Error::InvalidCString(_) => {
                    error!("InvalidCString: {:#?}", error);
                    Err(Status::InternalServerError)
                }
                Error::DatabaseError(_, _) => {
                    error!("DatabaseError: {:#?}", error);
                    Err(Status::InternalServerError)
                }
                Error::NotFound => {
                    warn!("Logging error: {:#?}", error);
                    Err(Status::NotFound)
                }
                Error::QueryBuilderError(_) => {
                    error!("QueryBuilderError: {:#?}", error);
                    Err(Status::InternalServerError)
                }
                Error::DeserializationError(_) => {
                    error!("DeserializationError: {:#?}", error);
                    Err(Status::InternalServerError)
                }
                Error::SerializationError(_) => {
                    error!("SerializationError: {:#?}", error);
                    Err(Status::InternalServerError)
                }
                Error::RollbackErrorOnCommit { .. } => {
                    error!("RollbackErrorOnCommit: {:#?}", error);
                    Err(Status::InternalServerError)
                }
                Error::RollbackTransaction => {
                    error!("RollbackTransaction: {:#?}", error);
                    Err(Status::InternalServerError)
                }
                Error::AlreadyInTransaction => {
                    error!("AlreadyInTransaction: {:#?}", error);
                    Err(Status::InternalServerError)
                }
                Error::NotInTransaction => {
                    error!("NotInTransaction: {:#?}", error);
                    Err(Status::InternalServerError)
                }
                Error::BrokenTransactionManager => {
                    error!("BrokenTransactionManager: {:#?}", error);
                    Err(Status::InternalServerError)
                }
                _ => {
                    error!("Uncached error: {:#?}", error);
                    Err(Status::InternalServerError)
                }
            };
        }
        Ok(task) => { Ok(json!(task)) }
    }
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

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    DbConn::get_one(&rocket).await
        .expect("Unable to retrieve connections")
        .run(|c| {
            c.run_pending_migrations(MIGRATIONS).expect("Migrations FAILED");
        }).await;

    rocket
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
        .attach(AdHoc::on_ignite("Diesel migrations", run_db_migrations))
        .register("/", catchers![not_found,unauthorized, unprocessable_entity])
        .launch()
        .await;
}
