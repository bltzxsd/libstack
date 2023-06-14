use crate::{
    db::establish_connection,
    loans::models::{create_loan, get_loan, return_book, LoanStatus, NewLoan},
};
use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[post("/loans/new")]
async fn new_loan(payload: web::Json<NewLoan>) -> impl Responder {
    let mut conn = establish_connection();
    match create_loan(payload, &mut conn).await {
        Ok(loan_id) => HttpResponse::Ok().json(loan_id),
        Err(e) => HttpResponse::InternalServerError().json(format!("{e}")),
    }
}

#[get("/loans/{loan_id}")]
async fn fetch_loan(loan_id: web::Path<uuid::Uuid>) -> impl Responder {
    let mut conn = establish_connection();

    match get_loan(*loan_id, &mut conn).await {
        Ok(Some(loan)) => HttpResponse::Ok().json(loan),
        Ok(None) => HttpResponse::NotFound().json(format!("loan {loan_id} not found")),
        Err(e) => HttpResponse::InternalServerError().json(format!("{e}")),
    }
}

#[delete("/loans/{loan_id}")]
async fn close_loan(
    loan_id: web::Path<uuid::Uuid>,
    status: web::Json<LoanStatus>,
) -> impl Responder {
    let mut conn = establish_connection();
    match return_book(*loan_id, *status, &mut conn).await {
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
