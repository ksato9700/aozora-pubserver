use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod book;
mod date_convert;
mod db_mongo;

use db_mongo::Db;

const API_ROOT: &str = "/api/v1";
const HOSTPORT: &str = "0.0.0.0:3000";

async fn get_books() -> impl Responder {
  "all books"
}

async fn get_book_by_id(book_id: web::Path<u32>) -> impl Responder {
  format!("Book id:{}", book_id)
}

async fn get_book_card(_book_id: web::Path<u32>) -> impl Responder {
  HttpResponse::NotImplemented()
}

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
fn main() -> () {
  let db: Db = Db::new();
  // print!("{:?}", db);

  println!("{}", db.find_one_book(123));

  // std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
  // env_logger::init();

  // HttpServer::new(|| {
  //   App::new().wrap(Logger::new("%a %r %s %b")).service(
  //     web::scope(API_ROOT)
  //       .service(web::resource("/books").to(get_books))
  //       .service(web::resource("/books/{id}").to(get_book_by_id))
  //       .service(web::resource("/books/{id}/card").to(get_book_card)),
  //   )
  // })
  // .bind(HOSTPORT)?
  // .run()
  // .await
}
