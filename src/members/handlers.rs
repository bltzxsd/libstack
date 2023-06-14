use crate::db::establish_connection;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use super::models::{add_member, delete_member, get_member, update_member, NewMember};

#[post("/members/new")]
async fn create_member(payload: web::Json<NewMember>) -> impl Responder {
    let mut conn = establish_connection();
    match add_member(&payload.name, &payload.email, payload.privilege, &mut conn) {
        Ok(member_id) => HttpResponse::Ok().json(member_id),
        Err(e) => HttpResponse::InternalServerError().json(format!("failed to create member {e}")),
    }
}

#[delete("/members/{member_id}")]
async fn remove_member(id: web::Path<uuid::Uuid>) -> impl Responder {
    let mut conn = establish_connection();
    match delete_member(*id, &mut conn) {
        Ok(_) => HttpResponse::Ok().json(format!("deleted {id}")),
        Err(e) => HttpResponse::NotFound().json(format!("{e}")),
    }
}

#[get("/members/{member_id}")]
async fn fetch_member(id: web::Path<uuid::Uuid>) -> impl Responder {
    let mut conn = establish_connection();
    match get_member(*id, &mut conn) {
        Ok(Some(member)) => HttpResponse::Ok().json(member),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().json(format!("failed to fetch member {e}")),
    }
}

#[put("/members/{member_id}")]
async fn change_member(
    id: web::Path<uuid::Uuid>,
    member_request: web::Json<NewMember>,
) -> impl Responder {
    let mut conn = establish_connection();
    match update_member(*id, member_request.into_inner(), &mut conn) {
        Ok(update) => {
            if update {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("failed to update member {e}")),
    }
}
