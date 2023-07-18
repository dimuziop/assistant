use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::PasswordHasher;
use chrono::Local;
use diesel::QueryResult;
use uuid::Uuid;
use crate::users::credentials::Credentials;
use crate::users::repositories::{UserRepository};
use crate::users::role::NewUserRole;
use crate::users::role_service::RoleService;
use crate::users::user::{NewUserDTO, User};

pub enum LocalErrors {
    BadRequest(String),
    CreationFailed(String),
    InternalError(String)
}

pub struct IdentityService<'a> {
    user_repository: &'a mut UserRepository,
    role_service: &'a mut RoleService<'a>,
}

impl<'a> IdentityService<'a> {
    pub fn new(user_repository: &'a mut UserRepository, role_service: &'a mut RoleService<'a>,) -> IdentityService<'a> {
        IdentityService {
            user_repository,
            role_service
        }
    }
}


impl ManagesUsers for IdentityService<'_> {
    fn create_user(&mut self, new_user_dto: NewUserDTO) -> Result<User, LocalErrors> {
        let user = User {
            id: Uuid::new_v4().to_string(),
            name: new_user_dto.name,
            last_name: new_user_dto.last_name,
            created_at: Local::now().naive_local(),
            updated_at: None,
            deleted_at: None,
        };

        let roles = self.role_service.get_roles_by_code(new_user_dto.roles.clone());

        if roles.len() != new_user_dto.roles.len() {
            log::info!("Uncreated roles picked: {:?}", new_user_dto.roles);
            return Err(LocalErrors::BadRequest("Wrong selected roles".to_string()));
        }

        let new_user_roles: Vec<NewUserRole> = roles.iter().map(|role| {
            NewUserRole {
                id: Uuid::new_v4().to_string(),
                role_id: role.id.to_owned(),
                user_id: user.id.clone(),
            }
        }).collect();

        let argon2 = argon2::Argon2::default();
        let salt = SaltString::generate(OsRng);

        let credentials = Credentials {
            id: Uuid::new_v4().to_string(),
            user_id: user.id.clone(),
            email: new_user_dto.email,
            password: argon2.hash_password(new_user_dto.password.as_bytes(), &salt.clone()).unwrap().to_string(),
            created_at: Local::now().naive_local(),
            updated_at: None,
            deleted_at: None,
        };

        let repo_create = self.user_repository.create(
            user,
            new_user_roles,
            credentials,
        );

        match repo_create {
            Ok(created_user) => Ok(created_user.clone()),
            Err(error) => {
                log::warn!("Create user failed {}", error);
                Err(LocalErrors::CreationFailed(format!("Create user failed {}", error)))
            }
        }
    }

    fn get_users(&self, search_pattern: Option<&String>, limit: i64) -> Result<Vec<User>, LocalErrors> {
        match search_pattern {
            None => self.user_repository.get_all(limit)
                .map(|read_users| read_users)
                .map_err(|e| LocalErrors::InternalError(format!("Get users, get all error: {:?}", e))),
            Some(pattern) => self.user_repository.find_all_by_name(pattern, limit)
                .map(|read_users| read_users)
                .map_err(|e| LocalErrors::InternalError(format!("Get users, get all error: {:?}", e)))
        }
    }

    fn delete(&self, id: String) {
        match self.user_repository.soft_delete(id) {
            Ok(_) => {println!("OK")}
            Err(_) => {println!("FAILED")}
        }
    }
}


pub trait ManagesUsers {
    fn create_user(&mut self, new_user_dto: NewUserDTO) -> Result<User, LocalErrors>;
    fn get_users(&self, search_pattern: Option<&String>, limit: i64) -> Result<Vec<User>, LocalErrors>;
    fn delete(&self, id: String);
}