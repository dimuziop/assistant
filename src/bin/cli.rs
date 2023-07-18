extern crate assistant;

use clap::{Arg, Command};
use assistant::framework::commands::{create_users, delete_users, list_users, get_roles_options};
use dialoguer::{Input, MultiSelect, Password};
use rocket::futures::StreamExt;
use validators::prelude::*;
use assistant::schema::credentials::email;
use assistant::users::value_objects::Email;

fn main() {
    let matches = Command::new("Assistant")
        .about("Assistant commands")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("User Management")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a user with roles")
                    /*.arg(Arg::new("email").required(true))
                    .arg(Arg::new("password").required(true))
                    .arg(Arg::new("name").required(true))
                    .arg(Arg::new("last_name").required(true))
                    .arg(Arg::new("roles").required(true).num_args(1..).value_delimiter(','))*/
                )
                .subcommand(
                    Command::new("list")
                        .about("List users")
                        .arg(Arg::new("limit").default_value("10"))
                        .arg(Arg::new("search"))
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete user by id")
                        .arg(Arg::new("id").required(true))
                )
        ).get_matches();

    match matches.subcommand() {
        None => {}
        Some(("users", sub_matchers)) => match sub_matchers.subcommand() {
            None => {}
            Some(("create", sub_matchers)) => {
                let mail: String = Input::new().with_prompt("User Email (user@domain.com)")
                    .validate_with(|input: &String| -> Result<(), &str> {
                        match Email::parse_str(input.clone()) {
                            Ok(_) => { Ok(()) }
                            Err(err) => { Err("Invalid email address") }
                        }
                    })
                    .interact_text().unwrap();

                let password: String = Password::new().with_prompt("Password").interact().unwrap();
                let name: String = Input::new().with_prompt("Name")
                    .interact_text().unwrap();
                let last_name: String = Input::new().with_prompt("Last Name")
                    .interact_text().unwrap();
                let items = get_roles_options();
                let Ok(chosen) = MultiSelect::new()
                    .items(&items)
                    .interact() else { todo!() };
                let role_codes = chosen.iter().map(|&index| items[index].clone()).collect();
                create_users(mail, password, name, last_name, role_codes);
                ()
            }
            Some(("list", sub_matchers)) =>
                list_users(
                    sub_matchers.get_one::<String>("limit").unwrap().to_owned().parse::<i64>().unwrap(),
                    sub_matchers.get_one::<String>("search").to_owned(),
                ),
            Some(("delete", sub_matchers)) => delete_users(
                sub_matchers.get_one::<String>("id").unwrap().to_owned(),
            ),
            _ => {}
        }
        _ => {}
    }
}