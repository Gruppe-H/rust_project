use clap::Parser;
use std::env;
use exam_project::{
    cli::{Cli, CommandExecutor as _},
    users_db::UserManager,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();
    let (conn_string, users_db, users_collection) = get_env_values_or_use_default();
    let um: UserManager = UserManager::new(&conn_string, &users_db, &users_collection)?;
    cli.command.execute(um)?;
    Ok(())
}

fn get_env_values_or_use_default() -> (String, String, String) {
    // Retrieve MongoDB connection details from environment variables
    let conn_string = env::var("MONGODB_URL").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let users_db = env::var("MONGODB_DATABASE").unwrap_or_else(|_| "rust".to_string());
    let users_collection = env::var("MONGODB_COLLECTION").unwrap_or_else(|_| "users".to_string());

    (conn_string, users_db, users_collection)
}
