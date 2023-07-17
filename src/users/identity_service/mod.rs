use chrono::Local;
use uuid::Uuid;
use crate::users::credentials::Credentials;
use crate::users::repositories::{UserRepository};
use crate::users::role::NewUserRole;
use crate::users::role_service::RoleService;
use crate::users::user::{NewUserDTO, User};

pub enum LocalErrors {
    BadRequest(String),
    CreationFailed(String)
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

        let credentials = Credentials {
            id: Uuid::new_v4().to_string(),
            user_id: user.id.clone(),
            email: new_user_dto.email,
            password: new_user_dto.password,
            created_at: Local::now().naive_local(),
            updated_at: None,
            deleted_at: None,
        };



        let repo_create = self.user_repository.create(
            user,
            new_user_roles,
            credentials,
        );
        //let err = self.role_service.attach_roles(new_user_roles).err();




        match repo_create {
            Ok(created_user) => Ok(created_user.clone()),
            Err(error) => {
                log::warn!("Create user failed {}", error);
                Err(LocalErrors::CreationFailed(format!("Create user failed {}", error)))
            }
        }
    }

    fn list_user(&self) {
        todo!()
    }

    fn delete_user(&self, id: String) {
        todo!()
    }
}


pub trait ManagesUsers {
    fn create_user(&mut self, new_user_dto: NewUserDTO) -> Result<User, LocalErrors>;
    fn list_user(&self);
    fn delete_user(&self, id: String);
}