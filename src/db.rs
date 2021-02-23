use mongodb::{
    bson::{doc, Document},
    error::Error,
    results::{InsertOneResult, UpdateResult},
    Client, Collection,
};
use std::env;

#[derive(Clone)]
pub struct DBService {
    coll: Collection,
}

impl DBService {
    pub async fn by_collection_name(name: &str) -> DBService {
        let db_client = Client::with_uri_str(&env::var("MONGO_URL").unwrap())
            .await
            .unwrap();
        let db = db_client.database("med_kit");
        let coll = db.collection(name);
        DBService { coll: coll }
    }

    pub async fn create(&self, doc: Document) -> Result<InsertOneResult, Error> {
        self.coll.insert_one(doc, None).await
    }

    pub async fn get(&self, filter: Document) -> Result<Option<Document>, Error> {
        self.coll.find_one(filter, None).await
    }

    pub async fn set(&self, query: Document, content: Document) -> Result<UpdateResult, Error> {
        self.coll
            .update_one(
                query,
                doc! {
                    "$set":content
                },
                None,
            )
            .await
    }
}
