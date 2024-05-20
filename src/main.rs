use clap::Parser;
use exam_project::{
    cli::{Cli, CommandExecutor as _},
    users_db::UserManager,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();
    let um: UserManager = UserManager::new("mongodb://localhost:27017", "rust", "users")?;
    cli.command.execute(um)?;
    Ok(())
}

fn get_env_values_or_use_default() {
    // return env variables
}
