use mongodb::{
    bson::doc,
    error::Error as MongoError,
    options::UpdateOptions,
    Client as MongoClient,
};
use serde_json::Value as JsonValue;

pub struct MongoPersist {
    coll: mongodb::Collection<mongodb::bson::Document>,
}

impl MongoPersist {
    pub fn new(client: MongoClient) -> Self {
        let db = client.database("tokio_server");
        let coll = db.collection("kv_store");
        Self { coll }
    }

    pub async fn set(&self, key: &str, value: Option<JsonValue>) -> std::result::Result<(), MongoError> {
        let val_str = value.map(|v| v.to_string()).unwrap_or_else(|| "null".to_string());
        let filter = doc! { "_id": key };
        let update = doc! { "$set": { "value": val_str } };
        self.coll.update_one(filter, update, UpdateOptions::builder().upsert(true).build()).await?;
        Ok(())
    }

    pub async fn get(&self, key: &str) -> std::result::Result<Option<String>, MongoError> {
        if let Some(doc) = self.coll.find_one(doc! { "_id": key }, None).await? {
            if let Some(s) = doc.get_str("value").ok() {
                return Ok(Some(s.to_string()));
            }
        }
        Ok(None)
    }
}
