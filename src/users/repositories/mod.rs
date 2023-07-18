use diesel::{PgConnection, QueryResult, RunQueryDsl};
use crate::schema::users;
use crate::users::user::{User};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::schema::credentials;
use crate::schema::roles;
use crate::schema::users_roles;
use crate::users::credentials::Credentials;
use crate::users::role::{NewUserRole, Role};
use chrono::Local;

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
                    let _ = diesel::insert_into(users_roles::table).values(new_user_roles).execute(trc);
                    let _ = diesel::insert_into(credentials::table).values(new_credentials).execute(trc);
                    user
                })
            }
            Err(_) => {
                panic!("HAndle this");
            }
        }
    }

    pub fn find_all_by_name(&self, pattern: &String, limit: i64) -> QueryResult<Vec<User>> {
        match self.conn.get() {
            Ok(mut pool) => {
                users::table
                    .filter(users::name.ilike(pattern))
                    .or_filter(users::last_name.ilike(pattern))
                    .filter(users::deleted_at.is_null())
                    .limit(limit).load::<User>(&mut pool)
            }
            Err(_) => {
                panic!("HAndle this");
            }
        }
    }

    pub fn get_all(&self, limit: i64) -> QueryResult<Vec<User>> {
        match self.conn.get() {
            Ok(mut pool) => {
                users::table.filter(users::deleted_at.is_null()).limit(limit).load::<User>(&mut pool)
            }
            Err(_) => {
                panic!("HAndle this");
            }
        }
    }

    pub fn soft_delete(&self, id: String) -> QueryResult<usize> {
        match self.conn.get() {
            Ok(mut connection) => {
                connection.transaction(|trc| {
                    let deletion_time = Local::now().naive_local();
                    let user = diesel::update(users::table)
                        .filter(users::id.eq(id.clone()))
                        .set(users::deleted_at.eq(deletion_time.clone()))
                        .execute(trc);
                    let _ = diesel::update(users_roles::table).filter(users_roles::user_id.eq(id.clone())).set(users_roles::deleted_at.eq(deletion_time.clone())).execute(trc);;
                    let _ = diesel::update(credentials::table).filter(credentials::user_id.eq(id)).set(credentials::deleted_at.eq(deletion_time.clone())).execute(trc);;
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
                diesel::insert_into(users_roles::table).values(new_user_roles).execute(&mut pool)
            }
            Err(_) => {
                panic!("HAndle this");
            }
        }
    }
}


