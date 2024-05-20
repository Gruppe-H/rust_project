use clap::Parser;
use std::env;
use exam_project::{
    cli::{Cli, CommandExecutor as _},
    users_db::UserManager,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();
    let (conn_string, users_db, users_collection) = get_env_values();
    let um: UserManager = UserManager::new(&conn_string, &users_db, &users_collection)?;
    cli.command.execute(um)?;
    Ok(())
}

fn get_env_values() -> (String, String, String) {
    // Retrieve MongoDB connection details from environment variables
    let conn_string = env::var("MONGODB_URL").expect("Missing environment variable MONGODB_URL");
    let users_db = env::var("MONGODB_DATABASE").expect("Missing environment variable MONGODB_DATABASE");
    let users_collection = env::var("MONGODB_COLLECTION").expect("Missing environment variable MONGODB_COLLECTION");

    (conn_string, users_db, users_collection)
}
