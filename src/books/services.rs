use actix_web::{get, put, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use super::models::{AppState, Books};


#[get("/books")]
async fn entries_books(
    data: web::Data<AppState>
) -> impl Responder {
    let books_entries = data.books_entries.lock().unwrap();
    let entries: Vec<_> = books_entries.lists.values().collect();
    HttpResponse::Ok().json(entries)
}



#[derive(Deserialize, Serialize)]
struct GetEntriesBook {
    pub isbn: String
}


#[get("/books/")]
async fn get_entries_book(data: web::Data<AppState>, params: web::Query<GetEntriesBook>) -> impl Responder {
    let books_entries = data.books_entries.lock().unwrap();

    match books_entries.lists.get(&params.into_inner().isbn) {
        Some(book) => HttpResponse::Ok().json(book),
        None => HttpResponse::InternalServerError().finish()
    }
}


#[post("/books")]
async fn create_books_entries(
    data: web::Data<AppState>,
    book: web::Json<Books>
) -> impl Responder {
    let mut books_entries = data.books_entries.lock().unwrap();

    let Books { title, isbn, author, date } = book.into_inner();
    books_entries.lists.insert(isbn.clone(), Books::new(title.clone(), isbn.clone(), author.clone(), date.clone()));


    match books_entries.lists.get(&isbn.clone()) {
        Some(book) => HttpResponse::Ok().json(book),
        None => HttpResponse::InternalServerError().finish()
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(entries_books)
        .service(get_entries_book)
        .service(create_books_entries);
}
