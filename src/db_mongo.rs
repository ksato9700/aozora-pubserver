use bson::{bson, doc, Document};
use mongodb::error::Result;
use mongodb::options::FindOneOptions;
use mongodb::{Client, Collection, Database};
use std::env;

use crate::book::Book;

#[derive(Clone, Debug)]
pub struct DbClient {
    client: Client,
}

impl DbClient {
    pub fn new() -> Self {
        let mongo_url = format!(
            "mongodb://{}{}:{}/aozora",
            env::var("AOZORA_MONGODB_CREDENTIAL").unwrap_or("".to_string()),
            env::var("AOZORA_MONGODB_HOST").unwrap_or("localhost".to_string()),
            env::var("AOZORA_MONGODB_PORT").unwrap_or("27017".to_string()),
        );
        // println!("mongo_url: {}", mongo_url);
        Self {
            client: Client::with_uri_str(&mongo_url).unwrap(),
        }
    }
    pub fn _db(self) -> Database {
        self.client.database("aozora")
    }

    fn _find_one(
        collection: Collection,
        filter: Option<Document>,
        options: Option<FindOneOptions>,
    ) -> Result<Option<Document>> {
        let mut options: FindOneOptions = options.unwrap_or(FindOneOptions {
            ..Default::default()
        });
        options.projection = Some(doc! {"_id": 0});
        collection.find_one(filter, options)
    }

    pub fn find_one_book(self, book_id: u32) -> Result<Option<Book>> {
        let books = self._db().collection("books");
        let filter = Some(doc! { "book_id": book_id});
        match Self::_find_one(books, filter, None) {
            Ok(result) => Ok(match result {
                Some(document) => bson::from_bson(bson::Bson::Document(document)).unwrap(),
                None => None,
            }),
            Err(error) => Err(error),
        }
    }
}
