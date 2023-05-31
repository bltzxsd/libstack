use crate::db::establish_connection;
use crate::members::models::update_member;
use actix_web::delete;
use actix_web::get;
use actix_web::put;
use actix_web::web;
use actix_web::Responder;
use actix_web::post;
use actix_web::HttpResponse;

use super::models::add_member;
use super::models::NewMember;
use super::models::delete_member;
use super::models::get_member;

#[post("/members")]
async fn create_member(payload: web::Json<NewMember>) -> impl Responder {
    let mut conn = establish_connection();
    match add_member(&payload.name, &payload.email, payload.privilege, &mut conn) {
        Ok(member_id) => HttpResponse::Ok().json(member_id),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/members/{member_id}")]
async fn remove_member(id: web::Path<uuid::Uuid>) -> impl Responder {
    let mut conn = establish_connection();
    match delete_member(*id, &mut conn) {
        Ok(_) => HttpResponse::Ok().json(format!("Deleted {id}")),
        Err(e) => HttpResponse::NotFound().json(format!("{e}")),
    }
}

#[get("/members/{member_id}")]
async fn fetch_member(id: web::Path<uuid::Uuid>) -> impl Responder {
    let mut conn = establish_connection();
    match get_member(*id, &mut conn) {
        Ok(Some(member)) => HttpResponse::Ok().json(member),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().json(format!("{e}")),
    }
}

#[put("/members/{member_id}")]
async fn change_member(
    id: web::Path<uuid::Uuid>,
    member_request: web::Json<NewMember>
) -> impl Responder {
    let mut conn = establish_connection();
    match update_member(*id, member_request.into_inner(), &mut conn) {
        Ok(update) => {
            if update { HttpResponse::Ok().finish() } else { HttpResponse::NotFound().finish() }
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("{e}")),
    }
}
