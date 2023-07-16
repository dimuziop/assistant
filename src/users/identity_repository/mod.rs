use diesel::{PgConnection, QueryResult, RunQueryDsl};
use uuid::Uuid;
use crate::schema::users;
use crate::users::user::{NewUserDTO, User};
use chrono::{Local, NaiveDateTime};
use diesel::dsl;
use diesel::prelude::*;

pub struct IdentityRepository<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> IdentityRepository<'a> {

    pub fn new(conn: &'a mut PgConnection) -> IdentityRepository<'a> {
        IdentityRepository {
            conn
        }
    }

    pub fn create_user(&mut self, new_user: NewUserDTO) -> QueryResult<User> {
        let user = User {
            id: Uuid::new_v4().to_string(),
            name: new_user.name,
            last_name: new_user.last_name,
            created_at: Local::now().naive_local(),
            updated_at: None,
            deleted_at: None,
        };
        diesel::insert_into(users::table).values(user).get_result(self.conn)
    }
}