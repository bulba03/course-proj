use actix_web::{web::{Data, Json, ReqData}, post, HttpResponse, Responder};
use serde::Deserialize;

use crate::{TokenClaims, db_utils::AppState, messages::{user_services::check_role::CheckUserRole, lessons::create_lesson::CreateLesson}, insertables::new_user::RoleEnum};

#[derive(Deserialize)]
pub struct CreateLessonBody {
    course_id: i32,
    name: String,
    description: String,
    resourse_link: String
}
#[post("/create")]
async fn create_lesson(state: Data<AppState>, body: Json<CreateLessonBody>, req_user: Option<ReqData<TokenClaims>>) -> impl Responder {
    match req_user {
        Some(user) => {
            let check_user_role_msg = CheckUserRole {
                user_id: user.id,
                needed_role: RoleEnum::Admin
            };

            match state.db.send(check_user_role_msg).await {
                Ok(Ok(info)) => println!("{}", format!("It's ok, user {} has role {:?}", info.id, info.role)),
                Ok(Err(er)) => println!("{}", format!("Something got wrong: {er}")),
                _ => return HttpResponse::InternalServerError().json("Something got wrong!")
            }
            let msg = CreateLesson{
                name: body.name.clone(),
                description: body.description.clone(),
                course_id: body.course_id,
                resource_link: body.resourse_link.clone()
            };
            match state.db.send(msg).await {
                Ok(Ok(data)) => return HttpResponse::Ok().json(data),
                Err(err) => return HttpResponse::BadRequest().json(format!("{err}")),
                _ => return HttpResponse::InternalServerError().json("Something got wrong!")
            }
            
        }
        _ => HttpResponse::Unauthorized().json("Unable to identify identity"),
        
    }
}