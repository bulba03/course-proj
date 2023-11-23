use actix::Message;

use diesel::QueryResult;
use crate::db_model::user::User;
#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct AuthUser{
    pub email: String,
    pub pass: String,
}