use std::{error::Error, sync::Arc};

use serde_json::Value;

use crate::{users::UserError, users_db::UserManager};

pub fn create_many(um: &UserManager, users_str: &str) -> Result<(), Box<dyn Error>> {
    let um = Arc::new(um);
    let parsed_json: Value = serde_json::from_str(users_str).map_err(UserError::from)?;
    if let Value::Array(objects) = parsed_json {
        for obj in objects {
            println!("{}", obj);
        }
    };

    Ok(())
}
