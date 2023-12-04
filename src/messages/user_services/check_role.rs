use actix::Message;

use diesel::QueryResult;
use crate::{db_model::user::User, insertables::new_user::RoleEnum};
#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct CheckUserRole {
    pub user_id: i32,
    pub needed_role: RoleEnum
}
