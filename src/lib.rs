pub mod schema;
pub mod tasks;
pub mod framework;
pub mod authorization;
pub mod users;

use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket_db_pools::{Database, deadpool_redis};
use rocket_sync_db_pools::database;
use crate::authorization::basic_auth::BasicAuth;

#[database("postgres")]
pub struct DbConn(PgConnection);

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub struct ServerState {
    pub db_pool: DbPool
}

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorisation_header(auth_header) {
                log::info!("Successful Auth user: {}", auth.username);
                return Outcome::Success(auth);
            }
        }
        log::info!("Unauthorized Request");
        Outcome::Failure((Status::Unauthorized, ()))
    }
}