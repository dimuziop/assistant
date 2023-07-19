use rocket::{post, Route, routes, State};
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::{Json, json};
use serde_json::Value;
use crate::authorization::authorization_service::AuthorizationService;

#[derive(serde::Deserialize)]
pub struct BasicCredentials {
    pub email: String,
    pub password: String,
}

#[post("/login", format="json", data="<credentials>")]
async fn login(credentials: Json<BasicCredentials>, authorization_service: &State<AuthorizationService>) -> Result<Created<Value>, Status> {

    match authorization_service.login_with_credentials(credentials.0).await {
        Ok(token) => {
            Ok(Created::new(format!("localhost/{}/{}", 5,3)).body(json!({
                "Bearer": token
            })))
        }
        Err(_) => {
            Err(Status::Unauthorized)
        }
    }

}

pub fn routes() -> Vec<Route> {
    routes![login]
}

