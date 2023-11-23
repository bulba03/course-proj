use diesel::Queryable;
use serde::Serialize;

use crate::insertables::new_user::RoleEnum;

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub role: RoleEnum,
}