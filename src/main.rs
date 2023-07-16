#[macro_use]
extern crate rocket;

mod schema;
mod tasks;
mod framework;

use crate::authorisation::basic_auth::BasicAuth;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::{json, Value};
use rocket::{Build, Request, Rocket};
use rocket::http::Status;
use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};
use log::{info};
use rocket::fairing::AdHoc;

pub mod authorisation;

#[database("postgres")]
struct DbConn(diesel::PgConnection);


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
        .mount("/tasks", tasks::tasks_controller::routes())
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Diesel migrations", run_db_migrations))
        .register("/", catchers![not_found,unauthorized, unprocessable_entity])
        .launch()
        .await;
}
