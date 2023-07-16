use rocket::http::Status;
use rocket::serde::json::json;
use serde::Serialize;
use serde_json::Value;
use diesel::result::Error;
use log::{warn, error};

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