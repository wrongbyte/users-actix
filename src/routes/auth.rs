use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use serde::Serialize;
use validator::Validate;

use crate::{
    auth::create_jwt,
    domain::user::{payload::LoginUserPayload, validation::format_error_msg},
    error::AppError,
    handlers::user::DynUserHandler,
};

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

pub(crate) fn auth_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/auth").route("/", web::post().to(login_user)));
}

async fn login_user(
    body: web::Json<LoginUserPayload>,
    handler: web::Data<DynUserHandler>,
) -> Result<HttpResponse, AppError> {
    let payload = body.into_inner();

    if let Err(e) = payload.validate() {
        return Err(AppError::bad_request(format_error_msg(e.field_errors())));
    }

    let user_uuid = handler.get_user_by_login(payload).await?;

    // TODO: refactor, im not sure if this logic should be at this layer
    let jwt_user = create_jwt(user_uuid).map_err(|_| AppError {
        message: "Internal Error".to_string(),
        r#type: crate::error::ErrorType::InternalError,
    })?;

    Ok(HttpResponse::Ok().json(AuthResponse { token: jwt_user }))
}
