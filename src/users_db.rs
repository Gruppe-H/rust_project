use crate::db::MongoCollManager;
use crate::users::{User, UserError};
use mongodb::bson::oid::ObjectId;
use mongodb::error::Error as MongoError;
use mongodb::options::{DeleteOptions, UpdateOptions};
use mongodb::sync::{Client, Collection};
use mongodb::{
    bson::{doc, Document},
    options::{FindOptions, InsertOneOptions},
};
use std::error::Error;
use std::fmt::Display;

const ID_FIELD: &'static str = "_id";
const SET_OPERATOR: &'static str = "$set";

pub struct UserManager {
    coll: Collection,
}

impl UserManager {
    pub fn new(uri_str: &str, db_name: &str, coll_name: &str) -> Result<Self, UserManagerError> {
        let mongo_client: Client = Client::with_uri_str(uri_str).map_err(UserManagerError::from)?;
        println!("Successfully connected to mongodb");
        let users_coll: Collection = mongo_client.database(db_name).collection(coll_name);
        Ok(UserManager { coll: users_coll })
    }
}

impl MongoCollManager for UserManager {
    type Error = UserError;
    type ReadResult = User;

    fn create(&self, user_str: &str) -> Result<(), Self::Error> {
        let mut user: User = User::from_string(user_str)?;
        let user_doc: Document = user_doc_from_user(&user)?;
        let options = InsertOneOptions::default();

        let result = self.coll.insert_one(user_doc, options)?;
        if let Some(user_id) = result.inserted_id.as_object_id() {
            user.set_user_id(user_id.to_owned());
        } else {
            return Err(UserError::ObjectIdError(
                "Failed to get inserted user ID".to_string(),
            ));
        }
        println!("Created a new user: {}", user);

        Ok(())
    }

    fn read(&self, filter: Document) -> Result<Vec<Self::ReadResult>, Self::Error> {
        let mut users: Vec<User> = Vec::new();
        let options = FindOptions::default();
        let cursor = self
            .coll
            .find(filter, options)
            .expect("Failed to get cursor");

        for result in cursor {
            let document = result.expect("Couldn't find document");
            let user: User = mongodb::bson::from_document(document)?;
            users.push(user);
        }

        Ok(users)
    }

    fn update(&self, id_str: &str, user_str: &str) -> Result<(), Self::Error> {
        let id = ObjectId::with_string(&id_str).map_err(UserError::from)?;
        let filter = doc! {ID_FIELD: id};
        let new_user: User = User::from_string(user_str)?;
        let new_user_doc: Document = user_doc_from_user(&new_user)?;
        let update = doc! {SET_OPERATOR: new_user_doc};
        let options = UpdateOptions::default();

        let result = self.coll.update_one(filter, update, options)?;
        if result.modified_count > 0 {
            println!(
                "Update successful: {} document(s) modified",
                result.modified_count
            );
        } else {
            println!("Update failed");
        }
        Ok(())
    }

    fn delete(&self, id_str: &str) -> Result<(), Self::Error> {
        let id = ObjectId::with_string(&id_str).map_err(UserError::from)?;
        let filter = doc! {ID_FIELD: id};
        let options = DeleteOptions::default();

        let result = self.coll.delete_one(filter, options)?;
        if result.deleted_count > 0 {
            println!(
                "Delete successful: {} document(s) deleted",
                result.deleted_count
            );
        } else {
            println!("Delete failed");
        }
        Ok(())
    }
}

fn user_doc_from_user(user: &User) -> Result<Document, UserError> {
    let user_doc = mongodb::bson::to_bson(&user)
        .map_err(UserError::from)?
        .as_document()
        .ok_or_else(|| UserError::SerializationError("Conversion to document failed".to_string()))?
        .to_owned();
    Ok(user_doc)
}

#[derive(Debug)]
pub enum UserManagerError {
    DatabaseError(String),
}

impl Display for UserManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            //the dereference is necessary to access the actual enum variant when pattern matching on a reference
            UserManagerError::DatabaseError(ref message) => {
                write!(f, "Encountered database error: {}", message)
            }
        }
    }
}

impl Error for UserManagerError {}

impl From<MongoError> for UserManagerError {
    fn from(error: MongoError) -> Self {
        UserManagerError::DatabaseError(error.to_string())
    }
}
