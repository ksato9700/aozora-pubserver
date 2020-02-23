use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod book;
mod date_convert;
mod db_mongo;

use book::{get_book_by_id, get_book_card, get_books};

const API_ROOT: &str = "/api/v1";
const HOSTPORT: &str = "0.0.0.0:3000";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
  env_logger::init();

  HttpServer::new(move || {
    App::new().wrap(Logger::new("%a %r %s %b")).service(
      web::scope(API_ROOT)
        .service(web::resource("/books").to(get_books))
        .service(web::resource("/books/{id}").to(get_book_by_id))
        .service(web::resource("/books/{id}/card").to(get_book_card)),
    )
  })
  .bind(HOSTPORT)?
  .run()
  .await
}
