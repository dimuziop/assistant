#[macro_use]
extern crate rocket;

extern crate assistant;

use rocket::serde::json::{json, Value};
use rocket::{Build, Rocket};
use rocket::fairing::AdHoc;

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

    assistant::DbConn::get_one(&rocket).await
        .expect("Unable to retrieve connections")
        .run(|c| {
            c.run_pending_migrations(MIGRATIONS).expect("Migrations FAILED");
        }).await;

    rocket
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/tasks", assistant::tasks::tasks_controller::routes())
        .attach(assistant::DbConn::fairing())
        .attach(AdHoc::on_ignite("Diesel migrations", run_db_migrations))
        .register("/", catchers![not_found,unauthorized, unprocessable_entity])
        .launch()
        .await;
}