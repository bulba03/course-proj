

use crate::{AppState,DbActor, DEFAULT_COST, messages::create_user::CreateUser};

use actix::Addr;
use actix_web::{post, web::{Data, Json}, HttpResponse, Responder};

use bcrypt::{hash};


use serde::{Deserialize};


#[derive(Deserialize)]
pub struct CreateUserBody {
    first_name: String,
    last_name: String,
    email: String,
    password: String
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