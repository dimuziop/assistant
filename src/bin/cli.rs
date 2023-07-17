extern crate assistant;

use clap::{Arg, ArgMatches, Command};
use assistant::framework::commands::{create_users, delete_users, list_users};
use assistant::users;

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
                        .arg(Arg::new("email").required(true))
                        .arg(Arg::new("password").required(true))
                        .arg(Arg::new("name").required(true))
                        .arg(Arg::new("last_name").required(true))
                        .arg(Arg::new("roles").required(true).num_args(1..).value_delimiter(','))
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
            Some(("create", sub_matchers)) => create_users(
                sub_matchers.get_one::<String>("email").unwrap().to_owned(),
                sub_matchers.get_one::<String>("password").unwrap().to_owned(),
                sub_matchers.get_one::<String>("name").unwrap().to_owned(),
                sub_matchers.get_one::<String>("last_name").unwrap().to_owned(),
                sub_matchers.get_many::<String>("roles").unwrap().map(|v| v.to_string()).collect(),
            ),
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