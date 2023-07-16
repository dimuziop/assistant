use rocket::http::Status;
use rocket::serde::json::json;
use serde::Serialize;
use serde_json::Value;
use diesel::result::Error;
use log::{warn, error};
use rocket::response::status::Created;

pub fn build_ok_json_response<T: Serialize>(tasks_responses: Result<T, Error>) -> Result<Value, Status> {
    match tasks_responses {
        Err(error) => handle_errors(error),
        Ok(task) => { Ok(json!(task)) }
    }
}

pub fn build_created_json_response<T: Serialize>(tasks_responses: Result<T, Error>, resource: &str, identifier: String) -> Result<Created<Value>, Status> {
    match tasks_responses {
        Err(error) => handle_errors(error),
        Ok(task) => { Ok(Created::new(format!("localhost/{}/{}", resource, identifier)).body(json!(task))) }
    }
}


fn handle_errors<T>(error: Error) -> Result<T, Status> {
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