use actix_web::{web::{ServiceConfig, self}, Responder, HttpResponse};

pub(crate) fn user_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/", web::post().to(create_user))
            .route("/{userId}", web::patch().to(update_user_by_id))
            .route("/{userId}", web::get().to(get_user_by_id))
            .route("/{username}", web::get().to(get_user_by_username))
            .route("/{userId}", web::delete().to(delete_user))
    );
}

async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("create_user")
}

async fn get_user_by_id() -> impl Responder {
    HttpResponse::Ok().body("get_user_by_id")
}

async fn update_user_by_id() -> impl Responder {
    HttpResponse::Ok().body("update_user_by_id")
}

async fn get_user_by_username() -> impl Responder {
    HttpResponse::Ok().body("get_user_by_username")
}

async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("delete_user")
}
