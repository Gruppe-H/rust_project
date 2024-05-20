use bson::oid::Error as ObjectIdError;
use mongodb::bson::oid::{Error as OidError, ObjectId};
use mongodb::bson::{de::Error as DeError, ser::Error as SerError};
use mongodb::error::Error as MongoError;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Error as SerJsonError;
use std::error::Error;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    user_id: Option<ObjectId>,
    name: String,
    age: i32,
    email: String,
    password: String,
    username: String,
}

impl User {
    pub fn from_string(json: &str) -> Result<Self, UserError> {
        serde_json::from_str(json).map_err(UserError::from)
    }

    pub fn set_user_id(&mut self, user_id: ObjectId) {
        self.user_id = Some(user_id);
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {{ id: {}, name: {}, age: {}, email: {}, username: {} }}",
            self.user_id
                .as_ref() //as_ref to not consume ownership
                .map_or("None".to_string(), |id| id.to_hex()),
            self.name,
            self.age,
            self.email,
            self.username
        )
    }
}

#[derive(Debug)]
pub enum UserError {
    IncorrectJson(String),
    InvalidObjectId(ObjectId),
    SerializationError(String),
    DeserializationError(String),
    ObjectIdError(String),
    MongodbError(String),
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UserError::IncorrectJson(ref message) => write!(f, "Invalid JSON input: {}", message),
            UserError::InvalidObjectId(ref id) => write!(f, "Invalid object id: {}", id),
            UserError::SerializationError(ref message) => {
                write!(f, "Serialization error: {}", message)
            }
            UserError::DeserializationError(ref message) => {
                write!(f, "Deserialization error: {}", message)
            }
            UserError::ObjectIdError(ref message) => write!(f, "Object ID error: {}", message),
            UserError::MongodbError(ref message) => write!(f, "Database error: {}", message),
        }
    }
}

impl Error for UserError {}

impl From<SerError> for UserError {
    fn from(err: SerError) -> Self {
        UserError::SerializationError(err.to_string())
    }
}

impl From<SerJsonError> for UserError {
    fn from(err: SerJsonError) -> Self {
        UserError::IncorrectJson(err.to_string())
    }
}

impl From<DeError> for UserError {
    fn from(err: DeError) -> Self {
        UserError::DeserializationError(err.to_string())
    }
}

impl From<ObjectIdError> for UserError {
    fn from(err: ObjectIdError) -> Self {
        UserError::ObjectIdError(err.to_string())
    }
}

impl From<OidError> for UserError {
    fn from(err: OidError) -> Self {
        UserError::ObjectIdError(err.to_string())
    }
}

impl From<MongoError> for UserError {
    fn from(error: MongoError) -> Self {
        UserError::MongodbError(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_user_from_correct_json() {
        let input = r#"
            {
                "name": "Caroline",
                "age": 26,
                "email": "cph-ch465@cphbusiness.dk",
                "password": "password123",
                "username": "carol"
            }
        "#;

        let expected_user = User {
            user_id: None,
            name: "Caroline".to_string(),
            age: 26,
            email: "cph-ch465@cphbusiness.dk".to_string(),
            password: "password123".to_string(),
            username: "carol".to_string(),
        };

        let created_user = User::from_string(input).unwrap();
        assert_eq!(created_user, expected_user);
    }

    #[test]
    #[should_panic(expected = "IncorrectJson")]
    fn create_user_from_incorrect_json() {
        let input = r#"{name: "Maria"}"#;
        User::from_string(input).unwrap();
    }
}
