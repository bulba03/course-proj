use actix::Message;

use diesel::QueryResult;
use crate::db_model::user::User;
#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String
}
#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct AuthUser{
    pub email: String,
    pub pass: String,
}