use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use http_problem::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{handlers::user::DynUserHandler, response::GenericResponse};

#[derive(Serialize, Deserialize)]
pub struct NewUserPayload {
    pub name: Option<String>,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserPayload {
    pub name: Option<String>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
}

pub(crate) fn user_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/", web::post().to(create_user))
            .route("/{userId}", web::patch().to(update_user_by_id))
            .route("/{nickname}", web::get().to(get_user_by_nickname))
            .route("/{userId}", web::delete().to(delete_user)),
    );
}

async fn create_user(
    body: web::Json<NewUserPayload>,
    handler: web::Data<DynUserHandler>,
) -> Result<HttpResponse> {
    let payload = body.into_inner();
    let new_user = handler.create_user(payload).await?;
    Ok(HttpResponse::Ok().json(new_user))
}

async fn update_user_by_id(
    params: web::Path<i32>,
    body: web::Json<UpdateUserPayload>,
    handler: web::Data<DynUserHandler>,
) -> Result<HttpResponse> {
    let id = params.into_inner();
    let payload = body.into_inner();
    handler.update_user_by_id(id, payload).await?;
    Ok(HttpResponse::Ok().into())
}

async fn get_user_by_nickname(
    params: web::Path<String>,
    handler: web::Data<DynUserHandler>,
) -> Result<HttpResponse> {
    let nickname = params.into_inner();
    let user = handler.get_user_by_nickname(nickname).await?;
    if let Some(existent_user) = user {
        return Ok(HttpResponse::Ok().json(existent_user));
    }
    Ok(HttpResponse::Ok().json(GenericResponse::not_found()))
}

async fn delete_user(handler: web::Data<DynUserHandler>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("delete_user"))
}
