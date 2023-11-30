

use crate::{AppState,DbActor, DEFAULT_COST, messages::{create_user::CreateUser, change_pass::ChangePass}, TokenClaims};

use actix::Addr;
use actix_web::{post, web::{Data, Json, ReqData}, HttpResponse, Responder};

use bcrypt::hash;


use serde::Deserialize;


#[derive(Deserialize)]
pub struct CreateUserBody {
    first_name: String,
    last_name: String,
    email: String,
    password: String
}
#[derive(Deserialize)]
pub struct ChangePassBody {
    prev_password: String,
    new_password: String,
}

#[post("/user")]
async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    let user: CreateUserBody = body.into_inner();

    let hash = hash(user.password, DEFAULT_COST).unwrap();
    let db: Addr<DbActor> = state.as_ref().db.clone();
    let msg = CreateUser {
        first_name: user.first_name,
        last_name: user.last_name,
        password: hash,
        email: user.email
    };
    match db.send(msg).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[post("/user/change_pass")]
async fn change_password(state: Data<AppState>, body: Json<ChangePassBody>, req_user: Option<ReqData<TokenClaims>>) -> impl Responder {
    match req_user {
        Some(user) => {
            let hash = hash(body.new_password.clone(), DEFAULT_COST).unwrap();
            let msg = ChangePass {
                user_id: user.id,
                new_pass: hash,
                prev_pass: body.prev_password.clone()
            };
            match state.db.send(msg).await {
                Ok(_) => return HttpResponse::Ok().json("Password Changed"),
                Err(err) => return HttpResponse::BadRequest().json(format!("{err}")),
            }
            
        }
        _ => HttpResponse::Unauthorized().json("Unable to identify identity"),
        
    }
}
