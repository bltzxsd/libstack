use crate::books::models::update_book;
use crate::db::establish_connection;
use actix_web::delete;
use actix_web::get;
use actix_web::put;
use actix_web::web;
use actix_web::Responder;
use actix_web::{ post, HttpResponse };

use super::models::add_book as create_book;
use super::models::NewBook;
use super::models::delete_book;
use super::models::get_book;

#[post("/books")]
async fn add_book(payload: web::Json<NewBook>) -> impl Responder {
    let mut connection = establish_connection();
    let isbn = &*payload.isbn.clone().unwrap_or("null".to_string());
    match
        create_book(
            &payload.title,
            &payload.author,
            payload.publication_year,
            isbn,
            payload.availability_status,
            &mut connection
        )
    {
        Ok(book_id) => HttpResponse::Ok().json(book_id),
        Err(e) => HttpResponse::InternalServerError().json(format!("{e}")),
    }
}

#[delete("/books/{book_id}")]
async fn remove_book(book_id: web::Path<uuid::Uuid>) -> impl Responder {
    let mut connection = establish_connection();

    match delete_book(*book_id, &mut connection) {
        Ok(_) => HttpResponse::Ok().json(format!("Deleted {book_id}")),
        Err(e) => HttpResponse::NotFound().json(format!("{e}")),
    }
}

#[get("/books/{book_id}")]
async fn fetch_book(book_id: web::Path<uuid::Uuid>) -> impl Responder {
    let mut conn = establish_connection();

    match get_book(*book_id, &mut conn) {
        Ok(Some(book)) => HttpResponse::Ok().json(book),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().json(format!("{e}")),
    }
}

#[put("/books/{book_id}")]
async fn change_book(
    book_id: web::Path<uuid::Uuid>,
    book_request: web::Json<NewBook>
) -> impl Responder {
    let mut conn = establish_connection();
    match update_book(*book_id, book_request.into_inner(), &mut conn) {
        Ok(updated) => {
            if updated { HttpResponse::Ok().finish() } else { HttpResponse::NotFound().finish() }
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("{e}")),
    }
}
