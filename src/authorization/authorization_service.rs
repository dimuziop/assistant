use std::ops::Add;
use argon2::{PasswordHash, PasswordVerifier};
use chrono::{Days, Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::authorization::authorization_controller::BasicCredentials;
use crate::users::identity_service::LocalErrors;
use crate::users::repositories::{CredentialsRepository, UserRepository};

pub struct AuthorizationService {
    credentials_repository: CredentialsRepository,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,         // Optional. Audience
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
}

impl AuthorizationService {
    pub fn new(credentials_repository: CredentialsRepository) -> AuthorizationService {
        AuthorizationService {
            credentials_repository,
        }
    }

    pub async fn login_with_credentials(&self, credentials: BasicCredentials) -> Result<String, LocalErrors> {
        let db_credentials = self.credentials_repository.get_credentials_by_email(credentials.email).await;
        match db_credentials {
            Ok(cred) => {
                let db_hash = PasswordHash::new(cred.password.as_str()).expect("adds");
                let argon2 = argon2::Argon2::default();
                if argon2.verify_password(credentials.password.as_bytes(), &db_hash).is_err() {
                    return Err(LocalErrors::BadRequest("Unaut".to_string()));
                };
                Ok(AuthorizationService::issue_json_web_token())
            }
            Err(_) => {
                Err(LocalErrors::BadRequest("Unaut".to_string()))
            }
        }
    }

    fn issue_json_web_token() -> String {
        let my_claims = Claims {
            aud: "assistant".to_string(),
            exp: Local::now().naive_local().add(Duration::hours(24)).timestamp() as usize,
            iat: Local::now().naive_local().timestamp() as usize,
            iss: "assistant".to_string(),
            nbf: Local::now().naive_local().timestamp() as usize,
            sub: "Carlitos".to_string(),
        };
        encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).expect("SECCO SECCO") //TODO
    }
}