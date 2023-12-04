use actix_web::{web::{Data, Json, ReqData}, post, HttpResponse, Responder};
use serde::Deserialize;

use crate::{TokenClaims, db_utils::AppState, messages::{courses::delete_course::DeleteCourse, user_services::check_role::CheckUserRole}, insertables::new_user::RoleEnum};

#[derive(Deserialize)]
pub struct DeleteCourseBody {
    course_id: i32
}
#[post("/delete")]
async fn delete_course(state: Data<AppState>, body: Json<DeleteCourseBody>, req_user: Option<ReqData<TokenClaims>>) -> impl Responder {
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
            let msg = DeleteCourse {
                course_id: body.course_id
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