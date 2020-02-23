use crate::date_convert;
use crate::db_mongo::DbClient;
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonSummary {
  person_id: i32,
  last_name: String,
  first_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
  book_id: i32,
  pub title: String,
  title_yomi: String,
  title_sort: String,
  subtitle: String,
  subtitle_yomi: String,
  original_title: String,
  first_appearance: String,
  ndc_code: String,
  font_kana_type: String,
  copyright: bool,

  #[serde(with = "date_convert")]
  release_date: DateTime<Utc>,
  #[serde(with = "date_convert")]
  last_modified: DateTime<Utc>,
  card_url: String,
  base_book_1: String,
  base_book_1_publisher: String,
  base_book_1_1st_edition: String,
  base_book_1_edition_input: String,
  base_book_1_edition_proofing: String,
  base_book_1_parent: String,
  base_book_1_parent_publisher: String,
  base_book_1_parent_1st_edition: String,
  base_book_2: String,
  base_book_2_publisher: String,
  base_book_2_1st_edition: String,
  base_book_2_edition_input: String,
  base_book_2_edition_proofing: String,
  base_book_2_parent: String,
  base_book_2_parent_publisher: String,
  base_book_2_parent_1st_edition: String,
  input: String,
  proofing: String,
  text_url: String,
  #[serde(with = "date_convert")]
  text_last_modified: DateTime<Utc>,
  text_encoding: String,
  text_charset: String,
  text_updated: i32,
  html_url: String,
  #[serde(with = "date_convert")]
  html_last_modified: DateTime<Utc>,
  html_encoding: String,
  html_charset: String,
  html_updated: i32,
  revisers: Option<Vec<PersonSummary>>,
  authors: Vec<PersonSummary>,
}

pub async fn get_books() -> impl Responder {
  HttpResponse::Ok().body("[{\"many\":\"books\"}]")
}

pub async fn get_book_by_id(book_id: web::Path<u32>) -> impl Responder {
  let cl = DbClient::new();
  match cl.find_one_book(*book_id) {
    Ok(book) => match book {
      Some(book) => HttpResponse::Ok().json(book),
      None => HttpResponse::Ok().body("{}".to_string()),
    },
    Err(error) => {
      println!("ERRROR: {:?}", error);
      HttpResponse::InternalServerError().finish()
    }
  }
}

pub async fn get_book_card(_book_id: web::Path<u32>) -> impl Responder {
  HttpResponse::NotImplemented().finish()
}
