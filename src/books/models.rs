use crate::schema::books;

use anyhow::Result;
use diesel::AsChangeset;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::Selectable;

use diesel::prelude::Insertable;
use diesel::prelude::Queryable;
use serde::Deserialize;
use serde::Serialize;
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
    conn: &mut PgConnection
) -> Result<uuid::Uuid> {
    let book = NewBook {
        title: title.to_string(),
        author: author.to_string(),
        publication_year,
        isbn: Some(isbn.to_string()),
        availability_status: available,
    };

    let book_id = diesel
        ::insert_into(books::table)
        .values(&book)
        .returning(books::book_id)
        .get_result(conn)?;
    Ok(book_id)
}

pub fn get_book(id: uuid::Uuid, conn: &mut PgConnection) -> Result<Option<Book>> {
    let book = books::table.filter(books::book_id.eq(id)).first::<Book>(conn).optional()?;

    Ok(book)
}

pub fn update_book(id: uuid::Uuid, payload: NewBook, conn: &mut PgConnection) -> Result<bool> {
    use crate::schema::books::dsl::*;

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
    let num_deleted = diesel
        ::delete(books::dsl::books.filter(books::book_id.eq(id)))
        .execute(conn)?;
    if !(num_deleted > 0) {
        return Err(anyhow::anyhow!("Could not delete book."));
    }
    Ok(())
}
