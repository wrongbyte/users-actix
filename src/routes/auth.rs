use actix_web::{web::{ServiceConfig, self}, HttpResponse};
use validator::Validate;

use crate::{domain::user::{payload::LoginUserPayload, validation::format_error_msg}, error::AppError, handlers::user::DynUserHandler};

pub(crate) fn auth_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/", web::post().to(login_user))
    );
}

async fn login_user(
    body: web::Json<LoginUserPayload>,
    handler: web::Data<DynUserHandler>,
) -> Result<HttpResponse, AppError> {
    let payload = body.into_inner();
    
    if let Err(e) = payload.validate() {
        return Err(AppError::bad_request(format_error_msg(e.field_errors())));
    }

    handler.get_user_by_login(payload).await?;

    Ok(HttpResponse::Ok().into())
}
