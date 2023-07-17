use diesel::QueryResult;
use diesel::result::Error;
use crate::users::repositories::RoleRepository;
use crate::users::role::{NewUserRole, Role};

pub struct RoleService<'a> {
    role_repository: &'a mut RoleRepository,
}

impl<'a> RoleService<'a> {
    pub fn new(role_repository: &'a mut RoleRepository) -> RoleService {
        RoleService {
            role_repository
        }
    }

    pub fn get_roles_by_code(&mut self, role_codes: Vec<String>) -> Vec<Role> {
        match self.role_repository.get_roles_by_code(role_codes.clone()) {
            Ok(roles) => roles,
            Err(err) => {
                match err {
                    Error::NotFound => {
                        log::warn!("Empty list of roles {:?}", role_codes);
                        return Vec::new();
                    }
                    _ => {
                        log::error!("Fatal on list of roles: {:?}", err);
                        panic!("{}", err);
                    }
                }
            }
        }
    }

    pub fn attach_roles(&mut self, new_user_roles: Vec<NewUserRole>) -> Result<(), String> {
        match self.role_repository.attach_users(new_user_roles.clone()) {
            Ok(inserts) => {
                if inserts != new_user_roles.len() {
                    log::error!("Attached roles mismatch insert");
                    return Err("Missmatch inserts".to_string());
                }
                Ok(())
            }
            Err(err) => {
                log::error!("Fatal on list of roles: {:?}", err);
                panic!("{}", err);
            }
        }
    }
}