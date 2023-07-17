use diesel::{PgConnection, QueryResult, RunQueryDsl};
use crate::schema::users;
use crate::users::user::{User};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::schema::credentials::dsl::credentials;
use crate::schema::roles;
use crate::schema::users_roles::dsl::users_roles;
use crate::users::credentials::Credentials;
use crate::users::role::{NewUserRole, Role};

pub struct UserRepository {
    conn: Pool<ConnectionManager<PgConnection>>,
}

impl UserRepository {
    pub fn new(conn: Pool<ConnectionManager<PgConnection>>) -> UserRepository {
        UserRepository {
            conn
        }
    }

    pub fn create(&self, new_user: User, new_user_roles: Vec<NewUserRole>, new_credentials: Credentials) -> QueryResult<User> {
        match self.conn.get() {
            Ok(mut connection) => {
                connection.transaction(|trc| {
                    let user = diesel::insert_into(users::table).values(new_user).get_result(trc);
                    let _ = diesel::insert_into(users_roles).values(new_user_roles).execute(trc);
                    let _ = diesel::insert_into(credentials).values(new_credentials).execute(trc);
                    user
                })
            }
            Err(_) => {
                panic!("HAndle this");
            }
        }
    }
}

pub struct RoleRepository {
    conn: Pool<ConnectionManager<PgConnection>>,
}

impl RoleRepository {
    pub fn new(conn: Pool<ConnectionManager<PgConnection>>) -> RoleRepository {
        RoleRepository {
            conn
        }
    }

    pub fn get_roles_by_code(&self, codes: Vec<String>) -> QueryResult<Vec<Role>> {
        match self.conn.get() {
            Ok(mut pool) => {
                roles::table.filter(roles::code.eq_any(codes)).load::<Role>(&mut pool)
            }
            Err(_) => {
                panic!("HAndle this");
            }
        }
    }

    pub fn attach_users(&self, new_user_roles: Vec<NewUserRole>) -> QueryResult<usize> {
        match self.conn.get() {
            Ok(mut pool) => {
                diesel::insert_into(users_roles).values(new_user_roles).execute(&mut pool)
            }
            Err(_) => {
                panic!("HAndle this");
            }
        }
    }
}


