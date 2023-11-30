use actix_web::{web::{Data, Json, ReqData}, post, HttpResponse, Responder};
use bcrypt::hash;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct ChangePassBody {
    prev_password: String,
    new_password: String,
}

use crate::{db_utils::AppState, TokenClaims, DEFAULT_COST, messages::user_services::change_pass::ChangePass};


#[post("/change_pass")]
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