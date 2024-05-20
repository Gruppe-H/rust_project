use anyhow::Context as _;
use clap::{ArgGroup, Parser, Subcommand};
use mongodb::bson::doc;
use std::path::PathBuf;

use crate::{db::MongoCollManager as _, thread::create_many, users_db::UserManager};

#[derive(Parser)]
#[command(about = "A CLI for interacting with a database", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: UserCommands,
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// Reads all entries from the database. Takes optional flag --name / -n
    Read {
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Creates a new user entry in the database. Needs either the optional flag --user / -u or --file_path / -f
    #[command(group = ArgGroup::new("input").required(true).args(&["user", "file_path"]))]
    Create {
        #[arg(short, long)]
        user: Option<String>,
        #[arg(short, long)] //TODO
        file_path: Option<PathBuf>,
    },
    /// Updates a user entry in the database. Needs the flags --id and --user / -u
    Update {
        #[arg(long)]
        id: String,
        #[arg(short, long)]
        user: String,
    },
    /// Deletes a user entry in the database. Needs the flag --id
    Delete {
        #[arg(long)]
        id: String,
    },
}

pub trait CommandExecutor {
    fn execute(self, um: UserManager) -> Result<(), Box<dyn std::error::Error>>;
}

impl CommandExecutor for UserCommands {
    fn execute(self, um: UserManager) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            UserCommands::Read { name } => {
                let filter = if let Some(name) = name {
                    println!("Trying to find users with name: {}", name);
                    doc! {"name": name}
                } else {
                    println!("Reading all users in database");
                    doc! {}
                };

                let users = um.read(filter)?;
                users.iter().for_each(|user| println!("{}", user));
            }
            UserCommands::Create { user, file_path } => {
                if let Some(user) = user {
                    println!("Trying to create new user");
                    um.create(&user)?;
                } else if let Some(file_path) = file_path {
                    println!("Trying to read file from path: `{}`", file_path.display());
                    let file_str = std::fs::read_to_string(&file_path).with_context(|| {
                        format!("Could not read file from path: `{}`", file_path.display())
                    })?;
                    create_many(&um, &file_str)?;
                    //todo!();
                } else {
                    eprintln!("Needs --user flag or --file_path")
                }
            }
            UserCommands::Update { id, user } => {
                println!("Trying to update user with ID {} to: {}", id, user);
                um.update(&id, &user)?;
            }
            UserCommands::Delete { id } => {
                println!("Trying to delete user with ID {}", id);
                um.delete(&id)?;
            }
        }
        Ok(())
    }
}
