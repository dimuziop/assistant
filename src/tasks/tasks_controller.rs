use rocket::Route;
use crate::authorisation::basic_auth::BasicAuth;
use crate::{DbConn, NewTaskDTO};
use crate::framework::router_helpers::build_response_from_db_result;
use crate::tasks::tasks_repository::TasksRepository;
use rocket::http::Status;
use serde_json::Value;
use diesel::result::Error;
use rocket::serde::json::{Json, json};
use crate::tasks::task::Task;

#[get("/")]
async fn get_all_tasks(_auth: BasicAuth, db: DbConn) -> Result<Value, Status> {
    db.run(|c| {
        let db_result = TasksRepository::all(c, 1000);
        build_response_from_db_result(db_result)
    }).await
}

#[get("/<id>")]
async fn get_task(id: String, _auth: BasicAuth, db: DbConn) -> Result<Value, Status> {
    db.run(|c| {
        let db_result: Result<Task, _> = TasksRepository::find(c, id);
        build_response_from_db_result(db_result)
    }).await
}

#[post("/", format = "json", data = "<new_task>")]
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

#[put("/<id>", format = "json", data = "<new_task>")]
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

#[delete("/<id>")]
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

pub fn routes() -> Vec<Route> {
    routes![
        get_all_tasks,
        get_task,
        create_task,
        update_task,
        delete_task,
    ]
}