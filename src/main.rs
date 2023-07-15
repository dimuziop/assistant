#[macro_use]
extern crate rocket;

use crate::authorisation::basic_auth::BasicAuth;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status;
use rocket::serde::json::{json, Value};
use rocket::serde::ser::SerializeStruct;
use rocket::serde::Serializer;
use rocket::{Request, State};
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use uuid::{Timestamp, Uuid};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TimeAmount {
    value: i32,
    unit: TimeUnits,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: String,
    title: String,
    description: Option<String>,
    estimated_time: Option<TimeAmount>,
    /*#[serde(with = "time::serde::rfc3339")]
    initial_times: Vec<Timestamp>,
    #[serde(with = "time::serde::rfc3339")]
    end_times: Vec<Timestamp>,*/
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
                if auth.username == "foo".to_string() && auth.password == "bar".to_string() {
                    return Outcome::Success(auth);
                }
            }
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[get("/tasks")]
fn get_all_tasks(task_db: &State<Vec<Task>>, _auth: BasicAuth) -> Value {
    json!(task_db.to_vec())
}

#[get("/tasks/<id>")]
fn get_task(id: String) -> Value {
    json!({ "id": id })
}

#[post("/tasks", format = "json")]
fn create_task() -> Value {
    json!("Hello, world ")
}

#[put("/tasks/<id>", format = "json")]
fn update_task(id: String) -> Value {
    json!({ "id": id })
}

#[delete("/tasks/<id>")]
fn delete_task(id: String) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!({"Error": "Resource not found"})
}

#[catch(401)]
fn unauthorized() -> Value {
    json!({"Error": "Unauthorized"})
}

#[rocket::main]
async fn main() {
    let n: Option<BasicAuth> = None;
    let mut tasks_db: Vec<Task> = Vec::new();
    tasks_db.push(Task::default());
    let _ = rocket::build()
        .manage(tasks_db)
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
        .register("/", catchers![not_found,unauthorized])
        .launch()
        .await;
}
