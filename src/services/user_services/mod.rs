use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub mod authentification;
pub mod change_password;
pub mod create_user;
pub mod create_admin;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users")
        .service(authentification::auth)
        .service(create_user::create_user)
        .service(create_admin::create_admin)
        .service(web::scope("")
                .wrap(HttpAuthentication::bearer(crate::validator))
                .service(change_password::change_password))
        
    );

}