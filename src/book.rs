use crate::date_convert;
use crate::db_mongo::DbClient;
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PersonSummary {
  person_id: i32,
  last_name: String,
  first_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Book {
  book_id: i32,
  title: String,
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

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::test;
  use actix_web::{body, App};
  use serde_json::from_slice;

  fn str_to_utc(v: &str) -> DateTime<Utc> {
    let format = "%Y-%m-%dT%H:%M:%SZ";
    DateTime::<Utc>::from_utc(NaiveDateTime::parse_from_str(v, &format).unwrap(), Utc)
  }

  #[actix_rt::test]
  async fn test_single_book_ok() {
    let mut app =
      test::init_service(App::new().route("/books/{book_id}", web::get().to(get_book_by_id))).await;
    let req = test::TestRequest::with_uri("/books/123").to_request();
    let res = test::call_service(&mut app, req).await;
    assert!(res.status().is_success());

    let content_type = res.headers().get("content-type".to_string()).unwrap();
    assert_eq!(content_type, "application/json");

    let expected = Book {
      book_id: 123,
      title: "大川の水".to_string(),
      title_yomi: "おおかわのみず".to_string(),
      title_sort: "おおかわのみす".to_string(),
      subtitle: "".to_string(),
      subtitle_yomi: "".to_string(),
      original_title: "".to_string(),
      first_appearance: "「心の花」1914（大正3）年4月".to_string(),
      ndc_code: "NDC 914".to_string(),
      font_kana_type: "新字新仮名".to_string(),
      copyright: false,
      release_date: str_to_utc("1999-01-11T00:00:00Z"),
      last_modified: str_to_utc("2014-09-17T00:00:00Z"),
      card_url: "https://www.aozora.gr.jp/cards/000879/card123.html".to_string(),
      base_book_1: "羅生門・鼻・芋粥".to_string(),
      base_book_1_publisher: "角川文庫、角川書店".to_string(),
      base_book_1_1st_edition: "1950（昭和25）年10月20日".to_string(),
      base_book_1_edition_input: "1985（昭和60）年11月10日改版38版".to_string(),
      base_book_1_edition_proofing: "1985（昭和60）年11月10日改版38版".to_string(),
      base_book_1_parent: "".to_string(),
      base_book_1_parent_publisher: "".to_string(),
      base_book_1_parent_1st_edition: "".to_string(),
      base_book_2: "".to_string(),
      base_book_2_publisher: "".to_string(),
      base_book_2_1st_edition: "".to_string(),
      base_book_2_edition_input: "".to_string(),
      base_book_2_edition_proofing: "".to_string(),
      base_book_2_parent: "".to_string(),
      base_book_2_parent_publisher: "".to_string(),
      base_book_2_parent_1st_edition: "".to_string(),
      input: "j.utiyama".to_string(),
      proofing: "かとうかおり".to_string(),
      text_url: "https://www.aozora.gr.jp/cards/000879/files/123_ruby_1199.zip".to_string(),
      text_last_modified: str_to_utc("2004-03-15T00:00:00Z"),
      text_encoding: "ShiftJIS".to_string(),
      text_charset: "JIS X 0208".to_string(),
      text_updated: 2,
      html_url: "https://www.aozora.gr.jp/cards/000879/files/123_15167.html".to_string(),
      html_last_modified: str_to_utc("2004-03-15T00:00:00Z"),
      html_encoding: "ShiftJIS".to_string(),
      html_charset: "JIS X 0208".to_string(),
      html_updated: 0,
      revisers: None,
      authors: vec![PersonSummary {
        person_id: 879,
        last_name: "芥川".to_string(),
        first_name: "竜之介".to_string(),
      }],
    };
    match res.response().body().as_ref() {
      Some(body::Body::Bytes(bytes)) => {
        let actual = from_slice::<Book>(bytes).unwrap();
        assert_eq!(actual, expected);
      }
      _ => panic!("Response error"),
    };

    // assert_eq!(body, "");
    // println!("{:?}", res.body());

    // let body = res.take_body();
    // println!("{:?}", body.as_ref().unwrap());
  }

  // #[actix_rt::test]
  // async fn test_index_not_ok() {
  //   let req = test::TestRequest::default().to_http_request();
  //   let resp = index(req).await;
  //   assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
  // }
}
