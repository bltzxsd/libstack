use std::io::Write;

use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    web,
};
use anyhow::Context;
use anyhow::Result;

use chrono::{DateTime, NaiveDate};
use diesel::{
    deserialize::FromSql,
    pg::{Pg, PgValue},
    prelude::{Insertable, Queryable},
    result::Error::NotFound,
    serialize::{IsNull, ToSql},
    AsChangeset, AsExpression, ExpressionMethods, FromSqlRow, OptionalExtension, PgConnection,
    QueryDsl, RunQueryDsl, Selectable,
};
use serde::{Deserialize, Serialize};

use crate::{
    books::models::{get_book, update_book_status},
    errors::LibError,
    members::models::{get_member, update_member, NewMember},
    schema::loans,
};

#[derive(Debug, Deserialize)]
pub struct NewLoan {
    pub member_id: uuid::Uuid,
    pub book_id: uuid::Uuid,
    pub due_date: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = loans)]
pub struct LoanRequest {
    pub member_id: uuid::Uuid,
    pub book_id: uuid::Uuid,
    pub loan_date: chrono::NaiveDate,
    pub due_date: chrono::NaiveDate,
    pub return_date: Option<chrono::NaiveDate>,
    pub status: LoanStatus,
}
#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Serialize, Deserialize, Clone, Copy)]
#[diesel(sql_type = crate::schema::sql_types::LoanStatus)]
pub enum LoanStatus {
    Open,
    Returned,
    Overdue,
}

impl ToSql<crate::schema::sql_types::LoanStatus, Pg> for LoanStatus {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match *self {
            LoanStatus::Open => out.write_all(b"open")?,
            LoanStatus::Returned => out.write_all(b"returned")?,
            LoanStatus::Overdue => out.write_all(b"overdue")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::LoanStatus, Pg> for LoanStatus {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"open" => Ok(LoanStatus::Open),
            b"returned" => Ok(LoanStatus::Returned),
            b"overdue" => Ok(LoanStatus::Overdue),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Queryable, Selectable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Loan {
    loan_id: uuid::Uuid,
    member_id: uuid::Uuid,
    book_id: uuid::Uuid,
    loan_date: chrono::NaiveDate,
    due_date: chrono::NaiveDate,
    return_date: Option<chrono::NaiveDate>,
    status: LoanStatus,
}

pub async fn create_loan(
    payload: web::Json<NewLoan>,
    conn: &mut PgConnection,
) -> Result<uuid::Uuid> {
    let book = get_book(payload.book_id, conn)?.ok_or(NotFound)?;
    if !book.availability_status {
        return Err(LibError::ActixError(
            ErrorBadRequest("Book is not available to loan").to_string(),
        )
        .into());
    }
    let member = match get_member(payload.member_id, conn) {
        Ok(Some(member)) => member,
        _ => {
            return Err(LibError::ActixError(
                ErrorBadRequest("Invalid member credentials").to_string(),
            )
            .into());
        }
    };

    let due_date =
        match chrono::naive::NaiveDateTime::from_timestamp_opt(payload.due_date as i64, 0) {
            Some(t) => t.date(),
            _ => {
                return Err(
                    LibError::Chrono(String::from("cannot convert timestamp to date")).into(),
                )
            }
        };

    let new_loan = LoanRequest {
        member_id: payload.member_id,
        book_id: payload.book_id,
        loan_date: chrono::Utc::now().date_naive(),
        due_date,
        return_date: None,
        status: LoanStatus::Open,
    };

    let update = NewMember {
        name: member.name,
        email: member.email,
        privilege: member.privilege,
        borrowed: member.borrowed + 1,
    };

    update_member(member.member_id, update, conn)?;

    update_book_status(payload.book_id, false, conn)?;

    let id = diesel::insert_into(loans::table)
        .values(&new_loan)
        .returning(loans::dsl::loan_id)
        .get_result(conn)?;

    Ok(id)
}

fn update_loan_status(
    loan_id: uuid::Uuid,
    status: LoanStatus,
    conn: &mut PgConnection,
) -> Result<usize> {
    diesel::update(loans::table.filter(loans::loan_id.eq(loan_id)))
        .set(loans::status.eq(status))
        .execute(conn)
        .with_context(|| {
            LibError::ActixError(
                ErrorInternalServerError("failed to update loan status").to_string(),
            )
        })
}

// close function
pub async fn return_book(
    payload: uuid::Uuid,
    status: LoanStatus,
    conn: &mut PgConnection,
) -> Result<bool> {
    match get_loan(payload, conn).await {
        Ok(Some(db_loan)) => {
            update_loan_status(db_loan.loan_id, status, conn)?;
            update_book_status(db_loan.loan_id, true, conn)?;
            if status == LoanStatus::Returned {
                update_loan_return_date(payload, DateTime::date_naive(&chrono::Utc::now()), conn)?;
            }
            let member = get_member(db_loan.member_id, conn)?
                .expect("invalid member credentials submitted to loan. Please verify member and loan databases");

            let borrowed = if member.borrowed <= 0 {
                member.borrowed
            } else {
                member.borrowed - 1
            };

            let update = NewMember {
                name: member.name,
                email: member.email,
                privilege: member.privilege,
                borrowed,
            };
            update_member(member.member_id, update, conn)?;
            Ok(true)
        }
        Ok(None) => Err(LibError::DbError(format!("loan {payload} not found")).into()),
        Err(e) => Err(LibError::DbError(e.to_string()).into()),
    }
    // TODO: Handle late fee calculations here.
}

pub async fn get_loan(id: uuid::Uuid, conn: &mut PgConnection) -> Result<Option<Loan>> {
    Ok(loans::table.find(id).first::<Loan>(conn).optional()?)
}

pub fn update_loan_return_date(
    id: uuid::Uuid,
    new_val: NaiveDate,
    conn: &mut PgConnection,
) -> Result<usize> {
    diesel::update(loans::table.filter(loans::loan_id.eq(id)))
        .set(loans::return_date.eq(&new_val))
        .execute(conn)
        .with_context(|| {
            LibError::ActixError(
                ErrorInternalServerError(format!("failed to update book {id} status")).to_string(),
            )
        })
}
