use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

use crate::{
    domain::user::payload::{NewUserPayload, UpdateUserPayload},
    error::AppError,
    handlers::user::DynUserHandler,
};

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
) -> Result<HttpResponse, AppError> {
    let payload = body.into_inner();
    let new_user = handler.create_user(payload).await?;
    Ok(HttpResponse::Ok().json(new_user))
}

async fn update_user_by_id(
    params: web::Path<i32>,
    body: web::Json<UpdateUserPayload>,
    handler: web::Data<DynUserHandler>,
) -> Result<HttpResponse, AppError> {
    let id = params.into_inner();
    let payload = body.into_inner();
    handler.update_user_by_id(id, payload).await?;
    Ok(HttpResponse::Ok().into())
}

async fn get_user_by_nickname(
    params: web::Path<String>,
    handler: web::Data<DynUserHandler>,
) -> Result<HttpResponse, AppError> {
    let nickname = params.into_inner();
    let user = handler.get_user_by_nickname(nickname).await?;
    Ok(HttpResponse::Ok().json(user))
}

async fn delete_user(
    params: web::Path<i32>,
    handler: web::Data<DynUserHandler>,
) -> Result<HttpResponse, AppError> {
    let id = params.into_inner();
    handler.delete_user(id).await?;
    Ok(HttpResponse::Ok().into())
}
