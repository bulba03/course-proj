use actix_web::{web::{Data, Json, ReqData}, post, HttpResponse, Responder};
use serde::Deserialize;

use crate::{TokenClaims, db_utils::AppState, messages::{courses::create_course::CreateCourse, user_services::check_role::CheckUserRole}, insertables::new_user::RoleEnum};

#[derive(Deserialize)]
pub struct CreateCourseBody {
    teacher_id: i32,
    name: String,
    description: String
}
#[post("/create")]
async fn create_course(state: Data<AppState>, body: Json<CreateCourseBody>, req_user: Option<ReqData<TokenClaims>>) -> impl Responder {
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
            let msg = CreateCourse {
                name: body.name.clone(),
                desctiption: body.description.clone(),
                teacher_id: body.teacher_id
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