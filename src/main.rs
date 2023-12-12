use actix_web::{get, web, App, HttpServer};
use std::sync::Mutex;

mod books;
use books::services;


#[get("/")]
async fn index() -> String {
    "Check endpoint".to_string()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_data = web::Data::new(books::models::AppState {
        books_entries: Mutex::new(books::models::BooksEntries::new())
    });

    HttpServer::new(move || {

        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
