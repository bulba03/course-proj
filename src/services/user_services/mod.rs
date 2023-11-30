use actix_web::web;

pub mod authentification;
pub mod change_password;
pub mod create_user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users")
        .service(authentification::auth)
        .service(create_user::create_user)
        .service(change_password::change_password)
    );
}