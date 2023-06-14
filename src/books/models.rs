use crate::{errors::LibError, schema::books};

use actix_web::error::ErrorInternalServerError;
use anyhow::Context;
use anyhow::Result;
use diesel::{
    prelude::{Insertable, Queryable},
    AsChangeset, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl,
    Selectable,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub publication_year: i32,
    pub isbn: Option<String>,
    pub availability_status: bool,
}

#[derive(Debug, Queryable, Selectable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Book {
    pub book_id: Uuid,
    pub title: String,
    pub author: String,
    pub publication_year: i32,
    pub isbn: Option<String>,
    pub availability_status: bool,
}

pub fn add_book(
    title: &str,
    author: &str,
    publication_year: i32,
    isbn: &str,
    available: bool,
    conn: &mut PgConnection,
) -> Result<uuid::Uuid> {
    let book = NewBook {
        title: title.to_string(),
        author: author.to_string(),
        publication_year,
        isbn: Some(isbn.to_string()),
        availability_status: available,
    };

    let book_id = diesel::insert_into(books::table)
        .values(&book)
        .returning(books::book_id)
        .get_result(conn)?;
    Ok(book_id)
}

pub fn update_book_status(id: uuid::Uuid, new_val: bool, conn: &mut PgConnection) -> Result<usize> {
    // match diesel::update(books::table.filter(books::book_id.eq(id)))
    //     .set(books::availability_status.eq(&new_val))
    //     .execute(conn)
    // {
    //     Ok(_) => HttpResponse::Ok().finish(),
    //     Err(e) => HttpResponse::InternalServerError()
    //         .json(format!("failed to update book {e} availability")),
    // }
    diesel::update(books::table.filter(books::book_id.eq(id)))
        .set(books::availability_status.eq(&new_val))
        .execute(conn)
        .with_context(|| {
            LibError::ActixError(
                ErrorInternalServerError(format!("failed to update book {id} status")).to_string(),
            )
        })
}

pub fn get_book(id: uuid::Uuid, conn: &mut PgConnection) -> Result<Option<Book>> {
    Ok(books::table
        .filter(books::book_id.eq(id))
        .first::<Book>(conn)
        .optional()?)
}

pub fn update_book(id: uuid::Uuid, payload: NewBook, conn: &mut PgConnection) -> Result<bool> {
    use crate::schema::books::dsl::{
        author, availability_status, book_id, books, isbn, publication_year, title,
    };

    let num_updated = diesel::update(books.filter(book_id.eq(id)))
        .set((
            title.eq(payload.title),
            author.eq(payload.author),
            publication_year.eq(payload.publication_year),
            isbn.eq(payload.isbn),
            availability_status.eq(payload.availability_status),
        ))
        .execute(conn)?;

    Ok(num_updated > 0)
}

pub fn delete_book(id: uuid::Uuid, conn: &mut PgConnection) -> Result<()> {
    let num_deleted: usize =
        diesel::delete(books::dsl::books.filter(books::book_id.eq(id))).execute(conn)?;
    if num_deleted == 0 {
        return Err(anyhow::anyhow!("Could not delete book."));
    }
    Ok(())
}
