use actix::SyncArbiter;
use actix_web::dev::{ServiceRequest, Service};
use actix_web::web::Data;
use actix_web::{App, HttpServer, web, HttpMessage, Error};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{BearerAuth, self};
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::pg::{PgConnection};
use diesel::r2d2::{self, ConnectionManager, Pool};
use dotenv::dotenv;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use serde::{Serialize, Deserialize};
use services::authentification::auth;
use services::user_services::create_user;
use sha2::Sha256;
use std::env;
mod db_utils;
mod services;
mod insertables;
mod schema;
mod messages;
mod actors;
mod db_model;
use db_utils::{get_pool, AppState, DbActor};
const DEFAULT_COST: u32 = 7;
#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    id: i32,
}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET IS NOT SET");
    let key: Hmac<Sha256> = Hmac::new_from_slice(&jwt_secret.as_bytes()).unwrap();
    let token_string = credentials.token();

    let claims: Result<TokenClaims, &str> = token_string.verify_with_key(&key).map_err(|_| "INVALID TOKEN");

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        },
        Err(_) => {
            let config = req.app_data::<bearer::Config>().cloned().unwrap_or_default().scope("");
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Загружаем переменные окружения из файла `.env`
    dotenv().ok();

    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));
    // Запускаем HTTP-сервер
    HttpServer::new(move || {
        let bearer_middleware = HttpAuthentication::bearer(validator);
        App::new()
        .app_data(Data::new(AppState {db: db_addr.clone()})).service(auth).service(create_user)
        
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}