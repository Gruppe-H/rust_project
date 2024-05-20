use clap::{Parser, Subcommand};
use mongodb::bson::doc;

use crate::{
    db::MongoCollManager as _,
    users_db::UserManager,
};

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
    /// Creates a new user entry in the database. Needs the flag --user / -u
    Create {
        #[arg(short, long)]
        user: String,
        //#[arg(short, long)] //TODO
        //file: Option<String>,
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
            UserCommands::Create { user } => {
                println!("Trying to create new user");
                um.create(&user)?;
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
