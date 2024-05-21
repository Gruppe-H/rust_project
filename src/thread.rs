use serde_json::Value;
use std::sync::Mutex;
use std::thread;
use std::{error::Error, sync::Arc};

use crate::db::MongoCollManager as _;
use crate::{users::UserError, users_db::UserManager};

struct JobsStatus {
    jobs_total: usize,
    jobs_completed: usize,
}

pub fn create_many(um: UserManager, users_str: &str) -> Result<(), Box<dyn Error>> {
    let um = Arc::new(um);
    let parsed_json: Value = serde_json::from_str(users_str).map_err(UserError::from)?;

    let status = Arc::new(Mutex::new(JobsStatus {
        jobs_total: 0,
        jobs_completed: 0,
    }));
    if let Value::Array(objects) = parsed_json {
        let status = Arc::clone(&status);
        status.lock().unwrap().jobs_total = objects.len();
        let handles: Vec<_> = objects
            .into_iter()
            .map(|obj| {
                let um = Arc::clone(&um);
                let status = Arc::clone(&status);
                let user_str = obj.to_string();

                thread::spawn(move || {
                    um.create(&user_str).unwrap();
                    let mut status = status.lock().unwrap();
                    status.jobs_completed += 1;
                    println!("Created {}/{}", status.jobs_completed, status.jobs_total);
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("Failed to join thread");
        }
    };

    Ok(())
}
