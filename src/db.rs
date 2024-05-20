use mongodb::bson::Document;

pub trait MongoCollManager {
    type Error;
    type ReadResult;

    fn create(&self, entity_str: &str) -> Result<(), Self::Error>;
    fn read(&self, filter: Document) -> Result<Vec<Self::ReadResult>, Self::Error>;
    fn update(&self, id: &str, entity_str: &str) -> Result<(), Self::Error>;
    fn delete(&self, id: &str) -> Result<(), Self::Error>;
}