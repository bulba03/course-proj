use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub mod create_course;
pub mod edit_course;
pub mod delete_course;
pub mod get_course;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/courses")
                .service(get_course::get_course)
                .service(web::scope("")
                .wrap(HttpAuthentication::bearer(crate::validator))
                .service(create_course::create_course)
                .service(delete_course::delete_course)
                .service(edit_course::edit_course))
                
                    
    );
}