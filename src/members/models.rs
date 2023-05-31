use anyhow::Result;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::Selectable;
use diesel::PgConnection;
use diesel::AsChangeset;
use diesel::prelude::Insertable;
use diesel::prelude::Queryable;

use serde::{ Deserialize, Serialize };
use uuid::Uuid;

use crate::schema::members;

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = members)]
pub struct NewMember {
    pub name: String,
    pub email: String,
    pub privilege: bool,
    pub borrowed: i32,
}

#[derive(Debug, Queryable, Selectable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Member {
    pub member_id: Uuid,
    pub name: String,
    pub email: String,
    pub borrowed: i32,
    pub privilege: bool,
}

pub fn add_member(
    name: &str,
    email: &str,
    privilege: bool,
    conn: &mut PgConnection
) -> Result<uuid::Uuid> {
    let member = NewMember {
        name: name.to_string(),
        email: email.to_string(),
        borrowed: 0,
        privilege,
    };

    let member_id = diesel
        ::insert_into(members::table)
        .values(&member)
        .returning(members::dsl::member_id)
        .get_result(conn)?;

    Ok(member_id)
}

pub fn get_member(id: uuid::Uuid, conn: &mut PgConnection) -> Result<Option<Member>> {
    Ok(members::table.filter(members::member_id.eq(id)).first(conn).optional()?)
}

pub fn update_member(id: Uuid, payload: NewMember, conn: &mut PgConnection) -> Result<bool> {
    use crate::schema::members::dsl::*;

    let num_updated = diesel
        ::update(members.filter(member_id.eq(id)))
        .set((name.eq(payload.name), email.eq(payload.email), borrowed.eq(payload.borrowed)))
        .execute(conn)?;

    Ok(num_updated > 0)
}

pub fn delete_member(id: uuid::Uuid, conn: &mut PgConnection) -> Result<()> {
    let num_deleted = diesel
        ::delete(members::dsl::members.filter(members::member_id.eq(id)))
        .execute(conn)?;
    if !(num_deleted > 0) {
        return Err(anyhow::anyhow!("Could not delete book."));
    }
    Ok(())
}
