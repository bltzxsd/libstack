use crate::{books::models::update_book, db::establish_connection};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use super::models::{add_book as create_book, delete_book, get_book, NewBook};

#[post("/books/new")]
async fn add_book(payload: web::Json<NewBook>) -> impl Responder {
    let mut connection = establish_connection();
    let isbn = &*payload.isbn.clone().unwrap_or("null".to_string());
    match create_book(
        &payload.title,
        &payload.author,
        payload.publication_year,
        isbn,
        payload.availability_status,
        &mut connection,
    ) {
        Ok(book_id) => HttpResponse::Ok().json(book_id),
        Err(e) => HttpResponse::InternalServerError().json(format!("{e}")),
    }
}

#[delete("/books/{book_id}")]
async fn remove_book(book_id: web::Path<uuid::Uuid>) -> impl Responder {
    let mut connection = establish_connection();

    match delete_book(*book_id, &mut connection) {
        Ok(_) => HttpResponse::Ok().json(format!("deleted {book_id}")),
        Err(e) => HttpResponse::NotFound().json(format!("{e}")),
    }
}

#[get("/books/{book_id}")]
async fn fetch_book(book_id: web::Path<uuid::Uuid>) -> impl Responder {
    let mut conn = establish_connection();

    match get_book(*book_id, &mut conn) {
        Ok(Some(book)) => HttpResponse::Ok().json(book),
        Ok(None) => HttpResponse::NotFound().json(format!("book {book_id} not found")),
        Err(e) => HttpResponse::InternalServerError().json(format!("{e}")),
    }
}

#[put("/books/{book_id}")]
async fn change_book(
    book_id: web::Path<uuid::Uuid>,
    book_request: web::Json<NewBook>,
) -> impl Responder {
    let mut conn = establish_connection();
    match update_book(*book_id, book_request.into_inner(), &mut conn) {
        Ok(updated) => {
            if updated {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("{e}")),
    }
}
