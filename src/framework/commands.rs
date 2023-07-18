use diesel::{Connection, PgConnection};
use crate::users::identity_service::{IdentityService, LocalErrors, ManagesUsers};
use crate::users::repositories::{RoleRepository, UserRepository};
use crate::users::role_service::RoleService;
use crate::users::user::{NewUserDTO, User};
use diesel::r2d2::{self, ConnectionManager};
use console::{style, Term};
use prettytable::{format, row, Table};

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load DB url from env");
    PgConnection::establish(&database_url).expect("Error connecting database")
}

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn create_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load DB url from env");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn create_users(email: String, password: String, name: String, last_name: String, roles: Vec<String>) {
    let pool = create_pool();
    let mut user_repository = UserRepository::new(pool.clone());
    let mut role_repository = RoleRepository::new(pool.clone());
    let mut role_service = RoleService::new(&mut role_repository);
    let mut identity_service = IdentityService::new(&mut user_repository, &mut role_service);
    let new_user = NewUserDTO {
        email,
        password,
        name,
        last_name,
        roles,
    };
    let user = identity_service.create_user(new_user);
    println!("User")
}

pub fn list_users(limit: i64, search_pattern: Option<&String>) {
    let pool = create_pool();
    let mut user_repository = UserRepository::new(pool.clone());
    let mut role_repository = RoleRepository::new(pool.clone());
    let mut role_service = RoleService::new(&mut role_repository);
    let mut identity_service = IdentityService::new(&mut user_repository, &mut role_service);

    let query_result = identity_service.get_users(search_pattern, limit);

    let mut table = Table::new();
    table.add_row(row![
        style("User ID").cyan().on_black(),
        style("Name").cyan(),
        style("Last Name").cyan(),
        style("Creation Date Name").cyan()
    ]);
    match query_result {
        Ok(users) => {
            users.iter().for_each(|user| {
                table.add_row(row![
                    style(&*user.id).on_black(),
                    &*user.name,
                    &*user.last_name,
                    user.created_at.format("%Y-%m-%d %H:%M:%S"),
                ]);
            })
        }
        Err(_) => {}
    }

    table.set_format(*format::consts::FORMAT_DEFAULT);

    // Print the table to stdout
    table.printstd();
}

pub fn delete_users(id: String) {
    let pool = create_pool();
    let mut user_repository = UserRepository::new(pool.clone());
    let mut role_repository = RoleRepository::new(pool.clone());
    let mut role_service = RoleService::new(&mut role_repository);
    let mut identity_service = IdentityService::new(&mut user_repository, &mut role_service);

    let users = identity_service.delete(id);
}

pub fn get_roles_options() -> Vec<String> {
    let pool = create_pool();
    let mut role_repository = RoleRepository::new(pool.clone());

    let Ok(roles) = role_repository.get_all() else { todo!() };

    roles.iter().map(|rol| rol.code.clone()).collect()
}