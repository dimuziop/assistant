use diesel::{Connection, PgConnection};
use crate::users::identity_repository::IdentityRepository;
use crate::users::user::NewUserDTO;

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load DB url from env");
    PgConnection::establish(&database_url).expect("Error connecting database")
}

pub fn create_users(email: String, password: String, name: String, last_name: String, roles: Vec<String>) {
    let mut c = load_db_connection();
    let mut repo = IdentityRepository::new(&mut c);
    let new_user = NewUserDTO {
        email,
        password,
        name,
        last_name,
        roles,
    };
    let user = repo.create_user(new_user).unwrap();
    println!("User")
}

pub fn list_users(_limit: u32, _search_pattern: Option<&String>) {}

pub fn delete_users(_id: String) {}