use serde_json::Value;
use std::thread;
use std::{error::Error, sync::Arc};

use crate::db::MongoCollManager as _;
use crate::{users::UserError, users_db::UserManager};

pub fn create_many(um: UserManager, users_str: &str) -> Result<(), Box<dyn Error>> {
    let um = Arc::new(um);
    let parsed_json: Value = serde_json::from_str(users_str).map_err(UserError::from)?;

    if let Value::Array(objects) = parsed_json {
        let handles: Vec<_> = objects
            .into_iter()
            .map(|obj| {
                let um = Arc::clone(&um);
                let user_str = obj.to_string();

                thread::spawn(move || {
                    um.create(&user_str).unwrap()
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("Failed to join thread");
        }
    };

    Ok(())
}
