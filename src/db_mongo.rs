use bson::{bson, doc, Document};
use mongodb::options::FindOneOptions;
use mongodb::{Client, Collection, Database};

use crate::book::Book;

#[derive(Debug)]
pub struct Db {
    db: Database,
}

impl Db {
    pub fn new() -> Self {
        let client = Client::with_uri_str("mongodb://localhost").unwrap();
        Self {
            db: client.database("aozora"),
        }
    }

    fn _find_one(
        collection: Collection,
        filter: Option<Document>,
        options: Option<FindOneOptions>,
    ) -> String {
        let mut options: FindOneOptions = options.unwrap_or(FindOneOptions {
            ..Default::default()
        });
        options.projection = Some(doc! {"_id": 0});
        match collection.find_one(filter, options) {
            Ok(result) => {
                let book: Book = bson::from_bson(bson::Bson::Document(result.unwrap())).unwrap();
                println!("{:?}", book);
                ()
                //     result
                //         .unwrap()
                //         .into_iter()
                //         .for_each(|item| println!("{:?}", item));
            }
            Err(e) => println!("ERRORR {}", e),
        };
        "ok".to_string()
    }

    pub fn find_one_book(self, book_id: u32) -> String {
        let books = self.db.collection("books");
        let filter = Some(doc! { "book_id": book_id});
        Self::_find_one(books, filter, None)
    }
}
