use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use http_problem::prelude::*;
use serde::Deserialize;

use crate::handlers::user::DynUserHandler;

#[derive(Deserialize)]
pub struct NewUserPayload {
    pub name: Option<String>,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
}

pub struct UpdateUserPayload {
    pub name: Option<String>,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
}

pub(crate) fn user_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/", web::post().to(create_user))
            .route("/{userId}", web::patch().to(update_user_by_id))
            .route("/{userId}", web::get().to(get_user_by_id))
            .route("/{nickname}", web::get().to(get_user_by_nickname))
            .route("/{userId}", web::delete().to(delete_user)),
    );
}

async fn create_user(
    body: web::Json<NewUserPayload>,
    handler: web::Data<DynUserHandler>,
) -> Result<HttpResponse> {
    let payload = body.into_inner();
    handler.create_user(payload).await?;
    Ok(HttpResponse::Ok().body("create_user"))
}

async fn get_user_by_id(handler: web::Data<DynUserHandler>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("get_user_by_id"))
}

async fn update_user_by_id(handler: web::Data<DynUserHandler>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("update_user_by_id"))
}

async fn get_user_by_nickname(handler: web::Data<DynUserHandler>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("get_user_by_username"))
}

async fn delete_user(handler: web::Data<DynUserHandler>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("delete_user"))
}
