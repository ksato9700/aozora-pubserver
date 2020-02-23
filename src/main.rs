use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

mod book;
mod date_convert;
mod db_mongo;

use db_mongo::DbClient;

const API_ROOT: &str = "/api/v1";
const HOSTPORT: &str = "0.0.0.0:3000";

async fn get_books() -> impl Responder {
    HttpResponse::Ok().body("[{\"many\":\"books\"}]")
}

async fn get_book_by_id(book_id: web::Path<u32>) -> impl Responder {
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

async fn get_book_card(_book_id: web::Path<u32>) -> impl Responder {
    HttpResponse::NotImplemented().finish()
}

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
