use std::env;

use crate::{AppState, TokenClaims, DEFAULT_COST};

use actix_web::{post, web::Data, HttpResponse, Responder};

use actix_web_httpauth::extractors::basic::BasicAuth;

use bcrypt::{hash,verify};
use hmac::{Hmac, Mac};

use jwt::SignWithKey;

use sha2::Sha256;

#[post("/auth")]
async fn auth(state: Data<AppState>, credentials: BasicAuth) -> impl Responder {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(env::var("JWT_SECRET").expect("JWT_SECRET IS NOT SET").as_bytes()).unwrap();
    let username = credentials.user_id();
    let _password = credentials.password();
    match _password {
        None => HttpResponse::BadRequest().json("No password provided"),
        Some(_password) => {
            let msg = crate::messages::auth_user::AuthUser{
                email: username.to_string(),
                pass: _password.to_string(),
            };
            match state.as_ref().db.clone().send(msg).await {
                Ok(Ok(info)) => {
                    let new_hash = hash(_password.to_string(), DEFAULT_COST).unwrap();
                    println!("{}", format!("{:?} {:?}", new_hash, info.password));
                    let is_valid = verify(_password.to_string(), info.password.as_str());
                    match is_valid {
                        Ok(true) => {
                            let claims = TokenClaims {id: info.id};
                            let token_str = claims.sign_with_key(&jwt_secret).unwrap();
                            HttpResponse::Ok().json(token_str)
                        }
                        Ok(false) => HttpResponse::BadRequest().json("Invalid password"),
                        Err(_) => HttpResponse::InternalServerError().json("Something went wrong"),
                    }
                }
                Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
                _ => HttpResponse::InternalServerError().json("Something went wrong"),
            }
        }
    }
}